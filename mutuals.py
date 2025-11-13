import sys
import json
import os


def find_user(fusername, filename="accounts.txt"):
    """ Find the user's line and return sets of followers/following. """

    base_dir = os.path.dirname(os.path.abspath(__file__))
    filepath = os.path.join(base_dir, filename)
    #Locates the file from any place in file directory

    with open(filepath, "r") as f: #opens the file for reading
        
        for fline in f: #iterates every line 
            fline = fline.strip()
            if not fline or ":" not in fline: #skips malformed lines 
                continue
                
            name, data = fline.split(":", 1) #splits only at the first colon
            
            if name.strip() == fusername: #checks if the data parsed is our desired username
                
                followers_str, following_str = data.split("|") #splits data at divider. left is followers, right is following
                
                f_followers = set(followers_str.split(",")) if followers_str else set() 
                f_following = set(following_str.split(",")) if following_str else set()
                #splits followers and following at the , and makes each an element of a set
                return f_followers, f_following
    return None, None  #returns nothing if the length of the file was exhausted


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

