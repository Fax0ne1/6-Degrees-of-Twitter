# get username from rust

# open userdata for reading

# read each line of file, checking first character for the username provided
#  if username in file
#  read lines into followers and following
#  generate sets for each
#  find intersection between them 
#  make new list with those usernames inside it.
#  send list back to rust

import sys
import json

def find_user_in_file(username, filename="accounts.txt"):
    """Find the user's line and return sets of followers/following."""
    with open(filename, "r") as f:
        for line in f:
            line = line.strip()
            if not line or ":" not in line:
                continue
            name, data = line.split(":", 1)
            if name.strip() == username:
                followers_str, following_str = data.split("|")
                followers = set(followers_str.split(",")) if followers_str else set()
                following = set(following_str.split(",")) if following_str else set()
                return followers, following
    return None, None  # not found

def main():
    for line in sys.stdin:
        username = line.strip()
        if username == "exit":
            break

        followers, following = find_user_in_file(username)
        if followers is None:
            print(json.dumps({"username": username, "error": "not found"}))
            sys.stdout.flush()
            continue

        mutuals = sorted(followers & following)
        print(json.dumps({"username": username, "mutuals": mutuals}))
        sys.stdout.flush()

if __name__ == "__main__":
    main()
