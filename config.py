import json, subprocess

with open("config.json", "r") as f:
    j = json.load(f)
    commit = subprocess.run(["git", "log", "-n", "1", "--pretty=format:\"%h\""], stdout=subprocess.PIPE).stdout
    j["commit"] = commit.decode("utf8").replace("\n", "").replace('"', "")
    with open("dist/config.json", "w") as f2:
        json.dump(j, f2)