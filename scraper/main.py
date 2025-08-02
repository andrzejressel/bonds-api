import json
import os
from multiprocessing import cpu_count, Pool
from typing import Dict, List, Tuple

import more_itertools
import pandas as pd
import requests
from bs4 import BeautifulSoup

import tables_extractor


def download_tabela_odsetkowa(file_id: str) -> str:
    file_name = f"tmp/tabela_odsetkowa_{file_id}.pdf"

    if os.path.exists(file_name):
        print(f"Plik {file_name} już istnieje, pomijam pobieranie.")
        return file_name

    # Make the request
    url = f"https://www.obligacjeskarbowe.pl/tabela-odsetkowa/?table_id={file_id}"
    response = requests.get(url)
    response.raise_for_status()

    # Parse with BeautifulSoup
    soup = BeautifulSoup(response.content, 'html.parser')

    span = soup.find("span", {"class": "files__text"})

    file_url = f"https://www.obligacjeskarbowe.pl{span.parent.attrs["href"]}"

    file_response = requests.get(file_url)
    file_response.raise_for_status()
    with open(file_name, 'wb') as file:
        file.write(file_response.content)
    print(f"Plik zapisany jako: {file_name}")
    # Return the file name

    # return

    return file_name


def extract_emission_numbers_and_data_values(url):
    """
    Wyciąga numery emisji obligacji i ich atrybuty data-value ze strony
    """
    response = requests.get(url, timeout=15)
    response.raise_for_status()

    # Parsowanie HTML
    soup = BeautifulSoup(response.content, 'html.parser')

    code_id_to_name = {}
    files_for_code_id: Dict[str, List[Tuple[str, str]]] = {}

    divs = soup.select("select#id_issue_bonds > option")

    for div in divs:
        if div.text.strip() == "Emisja":
            continue
        value_attr = div.attrs["value"]
        value_attr = value_attr.split(",")
        code_id_to_name[value_attr[0]] = div.text.strip()
        # code = div.text.strip()
        # if code.startswith("EDO"):
        #     print(div.prettify())
        # print(code)

    divs = soup.select("select#id_interest_table_bonds > option")

    for div in divs:
        value_attr = div.attrs["value"]
        if value_attr == '0':
            continue
        value_attr = value_attr.split(",")
        file_id = str(value_attr[0])
        code_id = str(value_attr[1])
        if files_for_code_id.get(code_id) is None:
            files_for_code_id[code_id] = []
        files_for_code_id[code_id].append((file_id, div.text.strip()))

    code_name_to_id = {v: k for k, v in code_id_to_name.items()}

    print(code_name_to_id)
    print(files_for_code_id)

    my_list = []

    for bond_number in code_name_to_id.keys():
        if not bond_number.startswith("EDO"):
            continue
        print(f"Creating file for {bond_number}")
        files = files_for_code_id[code_name_to_id[bond_number]]
        my_list.append((bond_number, files))
        pass

    with Pool(cpu_count()) as pool:
        pool.map(generate_bond_csv_2, my_list)

    # for result in results:
    #     print(result)

    # with open("output/files.json", "w") as f:
    #     json.dump([p[0] for p in my_list], f)
    #

    # for el1, el2 in more_itertools.windowed(files, 2):
    #     print(f"{el1[1]} -> {el2[1]}")

    # files.win

    # print(files)

def generate_bond_csv_2(arg) -> str:
    return generate_bond_csv(arg[0], arg[1])

def generate_bond_csv(bond_number: str, files) -> str:
    file_location = f"output/{bond_number}.json"
    try:
        if os.path.exists(file_location):
            return file_location
        dfs = [
            tables_extractor.extract_tables_from_pdf(download_tabela_odsetkowa(file_id))
            for file_id, _ in files
        ]
        df = pd.concat(dfs, axis=1)
        df = df.T.groupby(df.columns).first().T  # Usuwa duplikaty kolumn
        # df = df.groupby(df.columns, axis=1).first()  # Usuwa duplikaty kolumn
        df.columns = pd.to_datetime(df.columns, format='%m-%Y')
        df.columns = df.columns.to_period("M")
        # df.columns =  pd.to_period(df.columns, freq='M')
        df.sort_index(axis=1, inplace=True)
        df.index.name = "Day"
        df_long = df.reset_index().melt(
            id_vars='Day',
            var_name='Month',
            value_name='Value'
        )
        df_long['Date'] = pd.to_datetime(
            df_long['Month'].astype(str) + '-' + df_long['Day'].astype(str),
            errors='coerce'
        )
        df_long.dropna(subset=['Date', 'Value'], inplace=True)
        df_long.sort_values('Date', inplace=True)
        print(df)
        print(df_long)
        df_final_csv = df_long[['Date', 'Value']]
        first_date = df_final_csv['Date'].min().strftime('%Y-%m-%d')
        values = df_final_csv['Value'].tolist()
        result_json = {"first_date": first_date, "values": values}
        with open(file_location, "w") as f:
            json.dump(result_json, f)
        return file_location
        # print(result_json)
        # print(df_final_csv)
        # df_final_csv.to_csv(file_location, index=False)
        # return file_location
    except Exception as e:
        raise Exception(f"Failed to generate bond csv for number [{bond_number}] and files [{files}]") from e


# except requests.RequestException as e:
#     print(f"Błąd podczas pobierania strony: {e}")
#     return None
# except Exception as e:
#     print(f"Błąd podczas parsowania strony: {e}")
#     return None

# Główne wykonanie
if __name__ == "__main__":
    # Docelowy URL
    target_url = "https://www.obligacjeskarbowe.pl/tabela-odsetkowa"

    print(f"Wyciąganie numerów emisji i data-value z: {target_url}")
    print("Proszę czekać...\n")

    extract_emission_numbers_and_data_values(target_url)
