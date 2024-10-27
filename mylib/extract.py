# Step1

import requests

def extract(
    url="https://raw.githubusercontent.com/nogibjj/Mini_PJT_6_Complex-SQL-Query-for-a-MySQL-Database_ISL/refs/heads/main/data_raw/HR_1.csv",
    file_path="HR_1.csv",
    timeout=10
):
    try:
        response = requests.get(url, timeout=timeout)
        response.raise_for_status()  # Raises an error for bad status codes
        with open(file_path, "wb") as f:
            f.write(response.content)
        return file_path
    except (requests.exceptions.RequestException, OSError):
        return None

if __name__ == "__main__":
    extract()

