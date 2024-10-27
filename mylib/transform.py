# Step2

import sqlite3
import csv


def load(dataset="HR_1.csv"):
    # Open and read the dataset
    with open(dataset, newline="", encoding="utf-8", errors="ignore") as file:
        payload = csv.reader(file, delimiter=",")

        # Create a connection to SQLite3 database
        conn = sqlite3.connect("HR_1.db")
        c = conn.cursor()

        # Drop the existing table if it exists and create a new HR table
        c.execute("DROP TABLE IF EXISTS HR_1")
        c.execute(
            """
            CREATE TABLE HR_1 (
                EmployeeNumber INTEGER PRIMARY KEY,
                Age INTEGER,
                Attrition TEXT,
                BusinessTravel TEXT,
                DailyRate INTEGER,
                Department TEXT,
                DistanceFromHome INTEGER,
                Education INTEGER,
                EducationField TEXT,
                EmployeeCount INTEGER
            )
            """
        )

        # Skip the header and insert the first 10 columns of data into the table
        next(payload)  # Skip the header row

        for row in payload:
            # Ensure row has at least 10 columns
            if len(row) >= 10:
                trimmed_row = row[:10]
                try:
                    # Insert the row into the database
                    c.execute(
                        """
                        INSERT INTO HR_1 
                        (EmployeeNumber, Age, Attrition, BusinessTravel, DailyRate, Department, DistanceFromHome, 
                        Education, EducationField, EmployeeCount)
                        VALUES(?,?,?,?,?,?,?,?,?,?)
                        """,
                        trimmed_row,
                    )
                except sqlite3.Error as e:
                    print(f"Error inserting row: {row} with error {e}")

        # Commit and close the connection
        conn.commit()
        conn.close()

    return "HR_1.db"


if __name__ == "__main__":
    load()
