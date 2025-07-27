import typing

import camelot
from pypdf import PdfReader
import pandas as pd
import numpy as np

from dataclasses import dataclass


@dataclass
class PDFMetaData:
    Author: typing.Optional[str]
    Producer: typing.Optional[str]
    Creator: typing.Optional[str]


type1_pdf_metadata = PDFMetaData(
    Author='jaroslawkrolikowski',
    Producer='Microsoft® Excel® 2010',
    Creator='Microsoft® Excel® 2010'
)

type1_pdf_metadata_2 = PDFMetaData(Author='N1402192', Producer='GPL Ghostscript 9.05', Creator='PDFCreator Version 1.5.0')

type2_pdf_metadata = PDFMetaData(
    Author='Windows User',
    Producer='Microsoft® Excel® 2010',
    Creator='Microsoft® Excel® 2010'
)

type2_pdf_metadata_2 = PDFMetaData(
    Author='Windows User',
    Producer='Microsoft® Excel® 2016',
    Creator='Microsoft® Excel® 2016'
)

type2_pdf_metadata_3 = PDFMetaData(
    Author='Anna Godlewska',
    Producer='Microsoft® Excel® 2016',
    Creator='Microsoft® Excel® 2016'
)

type2_pdf_metadata_4 = PDFMetaData(Author=None, Producer=None, Creator=None)

type2_pdf_metadata_5 = PDFMetaData(Author='Małgorzata Panas', Producer='Microsoft® Excel® 2010',
                                   Creator='Microsoft® Excel® 2010')

type2_pdf_metadata_6 = PDFMetaData(Author='Joanna Szereda', Producer='Microsoft® Excel® 2010',
                                   Creator='Microsoft® Excel® 2010')

N1402316 = PDFMetaData(Author='N1402316', Producer='GPL Ghostscript 9.05',
                       Creator='PDFCreator Version 1.5.0')

type3_pdf_metadata = PDFMetaData(
    Author=None,
    Producer='iText 2.1.7 by 1T3XT',
    Creator='JasperReports Library version 6.14.0-2ab0d8625be255bf609c78e1181801213e51db8f'
)


def extract_tables_from_pdf(pdf_file: str) -> pd.DataFrame:
    reader = PdfReader(pdf_file)
    meta = reader.metadata

    pdf_metadata = PDFMetaData(
        Author=meta.get('/Author'),
        Producer=meta.get('/Producer'),
        Creator=meta.get('/Creator')
    )

    try:
        if pdf_metadata in [type1_pdf_metadata, type1_pdf_metadata_2]:
            return __extract_type1(pdf_file)
        elif pdf_metadata in [type2_pdf_metadata, type2_pdf_metadata_2, type2_pdf_metadata_3, type2_pdf_metadata_4,
                              type2_pdf_metadata_5, type2_pdf_metadata_6]:
            return __extract_type2(pdf_file)
        elif pdf_metadata == type3_pdf_metadata:
            return __extract_type3(pdf_file)
        elif pdf_metadata == N1402316 and 'TABELA ODSETKOWA OBLIGACJI' in reader.pages[0].extract_text():
            return __extract_type1(pdf_file)
        elif pdf_metadata == N1402316:
            return __extract_type2(pdf_file)
        else:
            raise Exception(f"Unsupported PDF metadata: [{pdf_metadata}] for file [{pdf_file}]")
    except Exception as e:
        raise Exception(f"Failed to parse file [{pdf_file}]") from e


def __extract_type1(pdf_file: str) -> pd.DataFrame:
    tables = camelot.read_pdf(pdf_file)
    if len(tables) == 0:
        raise Exception("No tables found in the PDF file.")
    df = tables[0].df

    # Use the first column as the new column headers
    df.columns = df.iloc[0]  # Promote the first row as header
    df = df[1:]  # Drop the original header row

    # Set the first column as the index or drop it
    df.index = df.iloc[:, 0]  # Set first column as index
    df = df.iloc[:, 1:]  # Drop the first column from the data

    months = {
        "STYCZEŃ": 1,
        "LUTY": 2,
        "MARZEC": 3,
        "KWIECIEŃ": 4,
        "MAJ": 5,
        "CZERWIEC": 6,
        "LIPIEC": 7,
        "SIERPIEŃ": 8,
        "WRZESIEŃ": 9,
        "PAŹDZIERNI": 10,
        "PAŹDZIERNIK": 10,
        "LISTOPAD": 11,
        "GRUDZIEŃ": 12
    }

    def rename_column(col: str):
        lines = col.splitlines()
        month = lines[0].strip()
        month = months[month]
        year = int(lines[1].strip())
        return f"{month:02d}-{year}"

    df.rename(columns=rename_column, inplace=True)

    df = df.map(lambda x: np.nan if x == '' else float(str(x).replace(',', '.')))
    df = df.apply(pd.to_numeric, errors='coerce')
    df.index = df.index.astype(int)
    df.index.name = None
    df.columns.name = None

    return df


def __extract_type2(pdf_file: str) -> pd.DataFrame:
    tables = camelot.read_pdf(pdf_file)
    if len(tables) == 0:
        raise Exception("No tables found in the PDF file.")
    df = tables[0].df

    # Use the first column as the new column headers
    df.columns = df.iloc[0]  # Promote the first row as header
    df = df[1:]  # Drop the original header row

    # Set the first column as the index or drop it
    df.index = df.iloc[:, 0]  # Set first column as index
    df = df.iloc[:, 1:]  # Drop the first column from the data

    def rename_column(col: str):
        lines = col.split("-")
        return f"{lines[1]}-{lines[0]}"

    df.rename(columns=rename_column, inplace=True)
    df = df.map(lambda x: np.nan if x == '' else float(str(x).replace(',', '.')))
    df = df.apply(pd.to_numeric, errors='coerce')
    df.index = df.index.astype(int)
    df.index.name = None
    df.columns.name = None

    return df


def __extract_type3(pdf_file: str) -> pd.DataFrame:
    tables = camelot.read_pdf(pdf_file, flavor='hybrid')
    if len(tables) == 0:
        raise Exception("No tables found in the PDF file.")

    df = tables[0].df

    # Use the first column as the new column headers
    df.columns = df.iloc[1]  # Promote the first row as header
    df = df[3:]  # Drop the original header row

    # Set the first column as the index or drop it
    df.index = df.iloc[:, 0]  # Set first column as index
    df = df.iloc[:, 1:]  # Drop the first column from the data

    def rename_column(col: str):
        lines = col.split("-")
        return f"{lines[1]}-{lines[0]}"

    df.rename(columns=rename_column, inplace=True)
    df = df.map(lambda x: np.nan if x == '' else float(str(x).replace(',', '.')))
    df = df.apply(pd.to_numeric, errors='coerce')
    df.index = df.index.astype(int)
    df.index.name = None
    df.columns.name = None

    return df
