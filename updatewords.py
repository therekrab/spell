import json

d = None

with open("words.json", "r") as i:
    d = set(json.load(i)["words"])

with open("words.txt", "r") as i:
    for line in i.readlines():
        d.add(line.strip())

with open("words.txt", "w") as o:
    for word in d:
        o.write(f"{word}\n")

print(f"updated all words. current word count: {len(d)}")
