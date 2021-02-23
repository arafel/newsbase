#!/usr/bin/env python
import sys

from Server import Server
from Group import Group

if len(sys.argv) < 2:
    print("Usage: get-headers.py group [group ...]")
    sys.exit(1)

s = Server("news.usenetserver.com")
s.connect()
for group in sys.argv[1:]:
    print("Group %s" % group)
    g = Group(group, s)
    g.open()
    g.close()
print("All done!")
