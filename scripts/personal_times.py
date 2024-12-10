import requests
import datetime

resp = requests.get(
    "https://adventofcode.com/2024/leaderboard/private/view/4409658.json",
    cookies={
        "session": "53616c7465645f5f3971e4f5ef3596e78bff1f5944f81a58399d10afea5ea573576ba93c971a14e897f7e27ca9b6c446b7e92d69a5cb1ede7d18904a3cd12590"
    },
)

data = resp.json()
max_days = 10
for member in data["members"].values():

    completed_days = member["completion_day_level"]
    stars = member["stars"]

    print(member["name"], len(completed_days))
    first_times = []
    second_times = []
    for d in range(1, max_days + 1):
        comp = member["completion_day_level"].get(str(d))
        if comp is None:
            first_times.append("--:--:--")
            second_times.append("--:--:--")
            continue
        if "1" not in comp:
            first_times.append("--:--:--")
        else:
            first = datetime.datetime.fromtimestamp(comp["1"]["get_star_ts"])
            first_times.append(first.time().isoformat())
        if "2" not in comp:
            second_times.append("--:--:--")
        else:
            second = datetime.datetime.fromtimestamp(comp["2"]["get_star_ts"])
            second_times.append(second.time().isoformat())

    print(first_times)
    print(second_times)
