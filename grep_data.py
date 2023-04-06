import sys
import socket
import json
import os
from datetime import date
import requests
from bs4 import BeautifulSoup as bs
from dataclasses import dataclass
import logging

LOG_LEVELS = {
    "CRITICAL": "🚨 CRITICAL",
    "ERROR": "❌ ERROR",
    "WARNING": "⚠️ WARNING",
    "INFO": "ℹ️ INFO",
    "DEBUG": "🐞 DEBUG",
}


logging.basicConfig(
    level=logging.DEBUG,
    format="%(asctime)s [%(levelname)s] %(message)s",
    datefmt="%Y-%m-%d %H:%M:%S",
)

[logging.addLevelName(logging.getLevelName(level), emoji)
 for (level, emoji) in LOG_LEVELS.items()]

SCHEME = "https://"
URL = SCHEME + "typeracerdata.com"
PARSER = "html.parser"

try:
    page = requests.get(URL)
except (
    ConnectionError,
    requests.exceptions.ConnectionError,
    socket.gaierror,
):
    print(" 😭 Website is down.")
    sys.exit(1)

soup = bs(page.content, PARSER)


@dataclass
class Profile:
    def __init__(
        self,
        rank,
        racer,
        text_bests,
        races,
        texts,
        career,
        best_10,
        best_race,
        points,
        wins,
        win_ratio,
        marathon,
        last_race,
        name):

        """ This consturctor defines basic profile attributes. """
        self.rank = rank
        self.racer = racer
        self.text_bests = text_bests
        self.races = races
        self.texts = texts
        self.career = career
        self.best_10 = best_10
        self.best_race = best_race
        self.points = points
        self.wins = wins
        self.win_ratio = win_ratio
        year, month, day = last_race.split("-", 3)
        year, month, day = int(year), int(month), int(day)
        self.marathon = marathon
        self.last_race = date(year, month, day)
        self.name = self.omit_bracks(name)

    def omit_bracks(self, string) -> str:
        """Remove brackets from a given string.

        :param string: The string to use when omitting the values.
        :return: `string` with omitted brackets.
        :rtype: str

        NOTE: If the string lacks given brackets `string` is returned.
        """

        if ")" in string:
            string.replace(")", "", 1)
        if "(" in string:
            string.replace("(", "", 1)
        return string


profiles = []
profiles_as_soup = soup.find_all("tr")

for profile_ in profiles_as_soup[1:]:
    profile_name = profile_.find("td", class_="l")
    profile_data_all = profile_.find_all(class_="r")
    profile_data_sum = [i.text for i in profile_data_all]
    profile_data_sum.insert(1, profile_name.text)
    profile = Profile(
            *profile_data_sum
            )
    profiles.append(profile)

profiles_as_hashmap = {}

for profile in profiles:
    profiles_as_hashmap[profile.name] = {
        "rank": profile.rank,
        "racer": profile.racer,
        "text_bests": profile.text_bests,
        "races": profile.races,
        "texts": profile.texts,
        "career": profile.career,
        "best_10": profile.best_10,
        "best_race": profile.best_race,
        "points": profile.points,
        "wins": profile.wins,
        "win_ratio": profile.win_ratio,
        "marathon": profile.marathon,
        "last_race": str(profile.last_race),
        "name": profile.name,
    }

file = os.path.join("src", "profiles.json")
if os.path.basename(file) in os.listdir(os.path.dirname(file)):
    print("Regenerating pre-existing data file.")
else:
    print("Generating data file fresh from source.")

json.dump(profiles_as_hashmap,
          open(os.path.join("src", "profiles.json"), "w"),
          sort_keys=True,
          ensure_ascii=True,
          indent=4)
