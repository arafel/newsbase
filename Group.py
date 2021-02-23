"""
Python module for working with NNTP groups.

Uses the Cache module to implement header caching, adding new article headers as required.
"""
import nntplib
import sqlite3
import json
import os
import email.header

from pathlib import Path
from dateutil import parser

def decode_header(header_str):
    """Takes a unicode string stresenting a munged header value
    and decodes it as a (possibly non-ASCII) readable value."""
    parts = []
    for v, enc in email.header.decode_header(header_str):
        if isinstance(v, bytes):
            try:
                parts.append(v.decode(enc or 'ascii'))
            except Exception as e:
                try:
                    print("Exception", e, "decoding", v, "with", enc, ", trying with iso-8859-1")
                    parts.append(v.decode('iso-8859-1'))
                except Exception as e:
                    print("Boom there too:", e)
                    print("Trying again with", enc, "but with surrogate-escape set")
                    parts.append(v.decode(enc or 'ascii', 'surrogateescape'))
        else:
            parts.append(v)
    return ''.join(parts)

class Group:
    """
    Python object for working with NNTP groups.

    Uses the Cache module to implement header caching, adding new article headers as required.
    """
    # pylint: disable=too-many-instance-attributes
    def __init__(self, groupname, server, nntp=None):
        if not server:
            raise Exception("No server supplied.")

        self._reversed_headers = []
        self._headers = []
        self._headercount = 0
        self._highwater = 0
        self._groupname = groupname
        self._cache_highwater = 0
        self._server = server
        p = Path("cache")
        db_path = p.joinpath("headers", groupname + ".db")
        print("Using db", db_path)
        os.makedirs(p.joinpath("headers", groupname), exist_ok=True)
        self._conn = sqlite3.connect(db_path, detect_types=sqlite3.PARSE_DECLTYPES|sqlite3.PARSE_COLNAMES)
        self._conn.row_factory = sqlite3.Row
        self._ensure_header_table()

    def __str__(self):
        return "Group '%s'" % self._groupname

    def _clean_headers(self, headers):
        if headers["date"].find("GMT Standard Time") >= 0:
            print("Replacing", headers["date"])
            headers["date"] = headers["date"].replace("GMT Standard Time", "GMT")
        if headers["date"].find("gmt") >= 0:
            print("Replacing", headers["date"])
            headers["date"] = headers["date"].replace("gmt", "GMT")
        if headers[":bytes"] == "":
            headers[":bytes"] = "0"
        return headers

    def _ensure_header_table(self):
        print("Creating 'headers' table")
        self._conn.execute('''create table if not exists headers(
                id INTEGER PRIMARY KEY,
                sender text,
                date timestamp,
                messageid text,
                refs text,
                subject text,
                bytes integer,
                xref text
                )
        ''')

    def convert_headers(self, item):
        (art_num, headers) = item
        headers = self._clean_headers(headers)
        try:
            new_obj = (
                decode_header(headers['subject']).encode('utf-8', 'surrogateescape').decode('ISO-8859-1'),
                decode_header(headers['from']).encode('utf-8', 'surrogateescape').decode('ISO-8859-1'),
                parser.parse(headers["date"]),
                headers["message-id"],
                headers["references"],
                int(headers[":bytes"]),
                headers["xref"],
                art_num
            )
            return new_obj
        except Exception as e:
            print("Converting", headers)
            print("Inserting 'id' of", art_num)
            print("Error!", e.__str__())
            raise e

    def get_headers(self, first, last):
        """
        Get headers #first to #last inclusive.
        """
        step = last - first
        print("Step %i (%i - %i)" % (step, last, first))
        if step > 100000:
            print("Header count is large, getting them in chunks.")
            step = 100000
        print("Getting headers from %i to %i, steps of %i" % (first, last, step))
        pos = first
        stepcount = ((last - first) / step) + 1

        while pos < last:
            cur = self._conn.cursor()
            if (pos + step) > last:
                step = last - pos

            print("%i chunk(s) remaining..." % stepcount)
            stepcount = stepcount - 1
            resp, overview = self._server.over((pos, pos + step - 1))
            if not resp.startswith("224"):
                print(resp)
                raise Exception("Couldn't get headers")

            # Convert date string to datetime object
            # for (art_num, headers) in overview:
            modified = map(self.convert_headers, overview)
            # data = self.convert_headers(art_num, headers)
            try:
                cur.executemany(
                    "insert into headers(subject, sender, date, messageid, refs, bytes, xref, id) values (?, ?, ?, ?, ?, ?, ?, ?)", modified
                )
            except Exception as e:
                print("Inserting data", modified)
                print("Error!", e.__str__())
                print(e)
                raise e

            pos = pos + step
            self._conn.commit()
            cur.close()

    def getMaxArticle(self):
        c = self._conn.cursor()
        c.execute("SELECT max(id) FROM headers")
        row = c.fetchone()
        if row[0] == None:
            return 0
        else:
            return row[0]

    def open(self):
        """
        Open the group, loading caches if available; checking the server
        to see if there are new articles, and adding those ot the cache.
        """
        self._highwater = self.getMaxArticle()
        self._cache_highwater = self._highwater
        print("Highwater now", self._highwater)

        # Get article limits from server
        (resp, count, first, last, name) = self._server.group(self._groupname)
        if not resp.startswith("211"):
            raise Exception("Couldn't select group")

        print("Server reports first", first, "last", last)

        if last > self._highwater:
            print("%i is higher than %i, get new articles" % (last, self._highwater))

            # Get new articles
            self.get_headers(self._highwater + 1, last)
            self._highwater = last
        else:
            print("No new articles.")

    def close(self):
        """ Close the group. """
        self._conn.close()

    def get_matching_numbers(self, pattern="", exclude=[], new_only=False, first=None, last=None):
        articles = []
        size = 0
        if new_only:
            if (first or last):
                raise Exception("Both new_only and first or last were set, you can't have both")
            if self._cache_highwater == self._highwater:
                print("No new items, nothing to check")
                return (articles, size, lines)
            if self._cache_highwater == 0:
                # Just initialising this group, start from the beginning.
                first = 0
            else:
                first = self._cache_highwater
            last = self.getMaxArticle()
        else:
            if not first:
                first = 0
            if not last:
                last = self.getMaxArticle()

        print("First", first, "last", last)
        if pattern:
            pattern = pattern.lower()
        if exclude:
            exclude = list(map(str.lower, exclude))

        headers = self._conn.execute("SELECT * FROM headers WHERE id>= first AND id <= last")
        print("Searching %i articles" % len(headers))
        articles = []
        size = 0
        # TODO do this in a more Pythonic way.
        for (art_num, over) in headers:
            try:
                subject = decode_header(over['subject']).lower()
            except UnicodeDecodeError:
                print("Error decoding header, skip")
                continue

            if subject.find(pattern) >= 0:
                skip = False
                for ex in exclude:
                    if subject.find(ex) >= 0:
                        skip = True
                        break

                if not skip:
                    articles.append(art_num)
                    # sizes = map(lambda x: int(x[1][":bytes"]), l2)
                    size = size + int(over[":bytes"])

        return (articles, size)

    def get_articles(self, filename, numbers, body_only=False):
        with open(filename, "wb") as fp:
            for num in numbers:
                try:
                    if body_only:
                        resp, info = self._server.body(num)
                    else:
                        resp, info = self._server.article(num)
                    fp.write(b'\n'.join(info.lines))
                except nntplib.NNTPTemporaryError:
                    print("Couldn't fetch article %i, skipping." % num)

    def get_subjects(self):
        subjects = []
        # for (art_num, over) in self._headers:
        cur = self._conn.cursor()
        cur.execute("select * from headers order by id")
        print("Fetching more rows")
        rows = cur.fetchmany()
        while len(rows) > 0:
            print("Got %i rows, processing" % len(rows))
            for (over) in rows:
                try:
                    subject = decode_header(over['subject']).encode('utf-8', 'surrogateescape').decode('ISO-8859-1')
                    subjects.append(subject)
                except UnicodeDecodeError:
                    print("Error decoding header, skip")
                    continue
            print("Fetching more rows")
            rows = cur.fetchmany()

        return subjects
