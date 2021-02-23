"""
Implement NNTP server functionality.
"""

import nntplib
from Cache import Cache

class Server:
    """
    Implement NNTP server functionality.
    """
    def __init__(self, host, nntp=None, use_ssl=True):
        """ Initialise. """
        self.host = host
        self.groups = []
        self.groups_loaded = False
        self.cache = Cache("server", host + "_groups")
        self.cached_date = None
        self.nntp = nntp
        self.use_ssl = use_ssl
        if nntp:
            self.connected = True
        else:
            self.connected = False

    def __str__(self):
        return "server (%s, connected=%s, ssl=%s)" % (self.host, self.connected, self.use_ssl)

    def get_host(self):
        """ Return hostname """
        return self.host

    def connect(self):
        """
        Connect to the server given at creation time.
        """
        try:
            print("Creating NNTP connection")
            if self.use_ssl:
                self.nntp = nntplib.NNTP_SSL(self.host, usenetrc=True)
            else:
                self.nntp = nntplib.NNTP(self.host, usenetrc=True)
            self.connected = True

            try:
                (self.cached_date, self.groups) = self.cache.load()
                print("Cache saved at", self.cached_date)
                print("Loaded %i groups from cache" % len(self.groups))
                self.groups_loaded = True
            except Exception:
                print("Couldn't load from cache, clear groups_loaded")
                self.groups_loaded = False
                self.cached_date = None
                self.groups = []

        # pylint: disable=broad-except
        except Exception:
            print("Couldn't connect.")
        return self.connected

    def disconnect(self):
        """
        Disconnect (if connected).
        """
        if self.connected:
            self.nntp.quit()

        self.connected = False

    def get_groups(self, get_all=False):
        """
        Get groups from the server, either all or new only (default).
        """
        if not self.connected:
            raise Exception("Need to connect first.")

        if not self.cached_date:
            print("No valid cache found, forcing get_all")
            get_all = True

        if get_all:
            print("Getting all groups")
            # Don't need to load the cache, we're going to destroy it anyway.
            resp, groups = self.nntp.list()
            if not resp.startswith("215"):
                print("Bad response to list", resp)
                raise Exception("Bad reply when getting groups")
            print("Got %i groups, saving to cache." % len(groups))
            self.cache.save(groups)
            print("Done.")
        else:
            print("Getting new groups")
            resp, groups = self.nntp.newgroups(self.cached_date)
            if not resp.startswith("231"):
                print("Bad response to newgroups", resp)
                raise Exception("Bad reply when getting groups")
            if len(groups) > 0:
                print("Got %i new groups" % len(groups))
                self.groups.extend(groups)
                self.cache.save(groups)
            else:
                print("No new groups.")

        self.groups_loaded = True

    def search_groups(self, pattern, raw_group=False):
        """
        Search groups for ones matching the given pattern.
        """
        if not self.groups_loaded:
            print("No groups loaded; is this the first time you've connected to the server?")
            print("If so you need to run get_groups()")
            raise Exception("No groups available to search, run get_groups()")

        if raw_group:
            return list(filter(lambda x: x.group.find(pattern) >= 0, self.groups))


        tmp = filter(lambda x: x.group.find(pattern) >= 0, self.groups)
        return list(map(lambda x: x.group, tmp))

    def body(self, arg):
        return self.nntp.body(arg)

    def article(self, arg):
        return self.nntp.article(arg)

    def over(self, arg):
        return self.nntp.over(arg)

    def group(self, arg):
        return self.nntp.group(arg)
