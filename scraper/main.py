import asyncio
import aiohttp
from bs4 import BeautifulSoup
from pathlib import Path
import os
import logging

BASE_URL = "https://www.obligacjeskarbowe.pl/tabela-odsetkowa/?table_id={}"
HEADERS = {
    "User-Agent": "Mozilla/5.0"
}
START = 11000
END = 11100
MAX_CONCURRENT_REQUESTS = 10
DOWNLOAD_DIR = Path("downloads")
DOWNLOAD_DIR.mkdir(exist_ok=True)

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s',
    handlers=[logging.StreamHandler()]
)

sem = asyncio.Semaphore(MAX_CONCURRENT_REQUESTS)

async def download_file(session, file_url, filename):
    try:
        async with session.get("https://www.obligacjeskarbowe.pl/" + file_url, timeout=20) as resp:
            if resp.status == 200:
                file_path = DOWNLOAD_DIR / filename
                with open(file_path, 'wb') as f:
                    while True:
                        chunk = await resp.content.read(1024)
                        if not chunk:
                            break
                        f.write(chunk)
                logging.info(f"Downloaded: {filename}")
            else:
                logging.warning(f"Failed to download {filename} (status: {resp.status})")
    except Exception as e:
        logging.error(f"Error downloading {filename}: {e}")

async def fetch(session, table_id):
    url = BASE_URL.format(table_id)
    async with sem:
        try:
            async with session.get(url, timeout=10) as response:
                if response.status != 200:
                    return None
                text = await response.text()
                soup = BeautifulSoup(text, "html.parser")
                span = soup.find("span", class_="files__text")
                link = span.parent
                if link and link.has_attr('href'):
                    file_url = link['href']
                    filename = f"table_{table_id}.pdf"
                    await download_file(session, file_url, filename)
                    return (table_id, file_url)
                else:
                    logging.warning(f"No download link found for table ID {table_id}")
                    return None
        except Exception as e:
            logging.error(f"[{table_id}] Error: {e}")
            return None

async def main():
    results = []
    async with aiohttp.ClientSession(headers=HEADERS) as session:
        tasks = [fetch(session, table_id) for table_id in range(START, END + 1)]
        for idx, future in enumerate(asyncio.as_completed(tasks), 1):
            logging.info(f"Starting download {idx} of {END - START + 1}")
            result = await future
            if result:
                results.append(result)
            else:
                logging.warning(f"Failed to fetch data for table ID {idx + START - 1}")
            logging.info(f"Finished download {idx} of {END - START + 1}")

    logging.info(f"\nTotal downloaded: {len(results)}")
    # Optional: save the links
    # with open("downloaded_links.csv", "w") as f:
    #     for tid, href in results:
    #         f.write(f"{tid},{href}\n")

if __name__ == "__main__":
    asyncio.run(main())