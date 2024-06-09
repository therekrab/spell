import json

d = None

with open("words.json", "r") as i:
    d = set(map(lambda s: s.lower(), json.load(i)["words"]))

# just to make sure that any custom words we added aren't deleted
with open("words.txt", "r") as i:
    for line in i.readlines():
        l = line.strip().lower()
        d.add(l)

with open("words.txt", "w") as o:
    for word in sorted(d):
        o.write(f"{word}\n")

print(f"updated all words. current word count: {len(d)}")
