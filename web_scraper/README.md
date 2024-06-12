# Web Scraper

```python
@app.local_entrypoint()
def main(urls: str):
    url_list = urls.split(',')
    for url in url_list:
        try:
            tables = get_tables(url)  # Direct call to get_tables
            print(f"Tables from {url}:\n")
            for table in tables:
                print(table)
                print("\n\n")
        except Exception as e:
            print(f"Error processing {url}: {e}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Receive multiple URLs as arguments.")
    parser.add_argument("urls", type=str, help="Comma-separated list of URLs to scrape.")

    args = parser.parse_args()
    main(args.urls)
```

1.  **Modal Integration**: The script uses Modal's `@app.local_entrypoint()` decorator to define the entry point for the Modal app. This makes it easy to run the script as a Modal job.
2.  **Argument Parsing**: The script uses `argparse` to accept a comma-separated list of URLs from the command line.
3.  **Main Function**: The `main` function splits the input URLs and processes each one to scrape tables.

## Running the Script with Modal

To run the script with Modal, you would typically deploy the app using Modal's CLI or API. Here’s a general outline of how you might do this:

```sh
 modal run ./web_scraper/__init__.py --urls "https://asn.flightsafety.org/wikibase/dblist2.php?yr=2024&at=&re=&pc=&op=&lo=&co=&ph=&na=&submit=Submit,https://asn.flightsafety.org/wikibase/dblist2.php?at=&re=&pc=&op=&fa=&lo=&co=&ph=&na=&yr=2024&page=1"
```

This will run the script on Modal’s infrastructure, leveraging its **serverless** compute resources to handle the _web scraping_ tasks.
