"""
    Title: mutuals.py
    Author: Morgan Quackenbush, Kalen Hazlett, Philipp Shchetinin

    Purpose: Receives an account via stdin, searches database for that account, 
    creates a set of the intersection between followers and following, and send that set back via stdout
"""


import sys
import json
import os
import sqlite3

def find_user(fusername, db_name="accounts.db"):
    """Find the user's line and return sets of followers/following (using a database)."""

    # Locate the DB file relative to this script
    base_dir = os.path.dirname(os.path.abspath(__file__)) #gets absolute path of mutuals.py
    db_path = os.path.join(base_dir, db_name) #creates a path to the database

    # Connect to the SQLite database
    conn = sqlite3.connect(db_path) #opens a connection to the db and connection object
    cur = conn.cursor() #creates a cursor to execute commands

    # Fetch followers
    cur.execute("SELECT follower FROM followers WHERE user = ?", (fusername,)) #Looks in table followers, uses ? to safely sub username
    followers = {row[0] for row in cur.fetchall()} #extracts the username from the tuple holding data

    # Fetch following
    cur.execute("SELECT follows FROM following WHERE user = ?", (fusername,))
    following = {row[0] for row in cur.fetchall()}

    conn.close() #closes connection to sql

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

