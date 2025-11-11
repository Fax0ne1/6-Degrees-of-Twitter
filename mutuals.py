import sys
import json
import os


def find_user(fusername, filename="accounts.txt"):
    """ Find the user's line and return sets of followers/following. """

    base_dir = os.path.dirname(os.path.abspath(__file__))
    filepath = os.path.join(base_dir, filename)

    with open(filepath, "r") as f:
        for fline in f:
            fline = fline.strip()
            if not fline or ":" not in fline:
                continue
            name, data = fline.split(":", 1)
            if name.strip() == fusername:
                followers_str, following_str = data.split("|")
                f_followers = set(followers_str.split(",")) if followers_str else set()
                f_following = set(following_str.split(",")) if following_str else set()
                return f_followers, f_following
    return None, None  # not found


for line in sys.stdin:
    username = line.strip()

    followers, following = find_user(username)
    if followers is None:
        print(json.dumps({"username": username, "error": "not found"}))
        sys.stdout.flush()
        continue

    mutuals = sorted(followers & following)
    print(json.dumps({"username": username, "mutuals": mutuals}))
    sys.stdout.flush()

