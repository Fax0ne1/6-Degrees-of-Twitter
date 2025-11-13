import sys
import json
import os
import sqlite3

def find_user(fusername, db_name="accounts.db"):
    """Find the user's line and return sets of followers/following (using a database)."""

    # Locate the DB file relative to this script
    base_dir = os.path.dirname(os.path.abspath(__file__))
    db_path = os.path.join(base_dir, db_name)

    # Connect to the SQLite database
    conn = sqlite3.connect(db_path)
    cur = conn.cursor()

    # Fetch followers
    cur.execute("SELECT follower FROM followers WHERE user = ?", (fusername,))
    followers = {row[0] for row in cur.fetchall()}

    # Fetch following
    cur.execute("SELECT follows FROM following WHERE user = ?", (fusername,))
    following = {row[0] for row in cur.fetchall()}

    conn.close()

    # If both empty, treat user as "not found" for output consistency
    if not followers and not following:
        return None, None

    return followers, following


for line in sys.stdin:
    username = line.strip()
    # reads in data from command line strips any white space

    followers, following = find_user(username)
    #sends data to find_user, puts that data into two different variables

    
    if followers is None:
        print(json.dumps({"username": username, "error": "not found"}))
        sys.stdout.flush()
        continue
    #if the data doesnt exist I.E. the username input wasnt found, returns an error to rust and flushes

    mutuals = followers & following
    #Finds the intersection of the two sets
    
    mutuals = sorted(mutuals)
    #Sorts the intersection
    
    print(json.dumps({"username": username, "mutuals": mutuals}))
    sys.stdout.flush()
    #sends found data back to rust 

