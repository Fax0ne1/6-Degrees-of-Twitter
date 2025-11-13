import sqlite3

conn = sqlite3.connect("accounts.db")
cur = conn.cursor()

cur.execute("CREATE TABLE IF NOT EXISTS users (username TEXT PRIMARY KEY)")
cur.execute("CREATE TABLE IF NOT EXISTS followers (user TEXT, follower TEXT)")
cur.execute("CREATE TABLE IF NOT EXISTS following (user TEXT, follows TEXT)")

cur.execute("CREATE INDEX IF NOT EXISTS idx_followers_user ON followers(user)")
cur.execute("CREATE INDEX IF NOT EXISTS idx_following_user ON following(user)")

with open("accounts.txt", "r") as f:
    for line in f:
        line = line.strip()
        if not line or ":" not in line:
            continue
        username, data = line.split(":", 1)
        followers_str, following_str = data.split("|")

        cur.execute("INSERT OR IGNORE INTO users VALUES (?)", (username.strip(),))

        # Insert followers
        if followers_str:
            for follower in followers_str.split(","):
                cur.execute("INSERT INTO followers VALUES (?, ?)", (username.strip(), follower.strip()))

        # Insert following
        if following_str:
            for follows in following_str.split(","):
                cur.execute("INSERT INTO following VALUES (?, ?)", (username.strip(), follows.strip()))

conn.commit()
conn.close()
print("✅ Migration complete! Data moved to accounts.db")
