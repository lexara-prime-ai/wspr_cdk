# Example [invocation]
#  modal run ./web_scraper/__init__.py --urls "https://asn.flightsafety.org/wikibase/dblist2.php?yr=2024&at=&re=&pc=&op=&lo=&co=&ph=&na=&submit=Submit,https://asn.flightsafety.org/wikibase/dblist2.php?at=&re=&pc=&op=&fa=&lo=&co=&ph=&na=&yr=2024&page=1"


import argparse
import urllib.request
from urllib.parse import urlparse

import modal
from bs4 import BeautifulSoup

app = modal.App(name="wspr-scraper-v0.0.1")


# URL format [validation].
def validate_url_scheme(url):
    ALLOWED_SCHEMES = {"http", "https"}
    SCHEME = urlparse(url).scheme
    if SCHEME not in ALLOWED_SCHEMES:
        raise ValueError(
            f"URL scheme '{SCHEME}' is not allowed. Allowed schemes: {ALLOWED_SCHEMES}"
        )


def get_tables(url):
    validate_url_scheme(url)  # Validate URL scheme before proceeding
    RESPONSE = urllib.request.urlopen(url)
    html = RESPONSE.read().decode("utf8")

    soup = BeautifulSoup(html, "lxml")
    tables = soup.find_all("table")

    TABLE_CONTENTS = []
    for table in tables:
        table_content = str(table)
        TABLE_CONTENTS.append(table_content)

    return TABLE_CONTENTS


@app.local_entrypoint()
def main(urls: str):
    url_list = urls.split(",")
    for url in url_list:
        try:
            # Direct call to get_tables.
            tables = get_tables(url)
            print(f"Tables from {url}:\n")
            for table in tables:
                print(table)
                print("\n\n")
        except Exception as e:
            print(f"Error processing {url}: {e}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Receive multiple URLs as arguments.")
    parser.add_argument(
        "urls", type=str, help="Comma-separated list of URLs to scrape."
    )

    args = parser.parse_args()
    main(args.urls)
