# Step 3
# """Query the database"""

import sqlite3


def queryCreate():
    conn = sqlite3.connect("HR_1.db")
    cursor = conn.cursor()
    # Ensure the table exists
    cursor.execute(
        """
        CREATE TABLE IF NOT EXISTS HR_1 (
            EmployeeNumber INTEGER UNIQUE,
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

    # Insert data into the correct table and with correct placeholders
    cursor.execute(
        """
        INSERT OR IGNORE INTO HR_1 
        (EmployeeNumber, Age, Attrition, BusinessTravel, DailyRate, Department, 
        DistanceFromHome, Education, EducationField, EmployeeCount)
        VALUES(99999, 30, 'Yes', 'Travel_Rarely', 1100, 'Sales', 
        5, 3, 'Life Sciences', 1)
        """
    )

    # Commit the transaction
    conn.commit()
    conn.close()
    return "Create Success"


def queryRead():
    conn = sqlite3.connect("HR_1.db")
    cursor = conn.cursor()

    # Read and fetch results from the HR table, limit the results
    cursor.execute("SELECT * FROM HR_1")
    # results = cursor.fetchall()  # Fetch the result of the query

    conn.close()
    return "Read Success"


def queryUpdate():
    conn = sqlite3.connect("HR_1.db")
    cursor = conn.cursor()

    # Update the data and commit the changes
    cursor.execute(
         """
        UPDATE HR_1
        SET 
            Age = CASE
                    WHEN Age IS NULL OR Age = '' THEN 9999999
                    ELSE Age
                END,
            Attrition = CASE
                    WHEN Attrition IS NULL OR Attrition = '' THEN '9999999'
                    ELSE Attrition
                END,
            BusinessTravel = CASE
                    WHEN BusinessTravel IS NULL OR BusinessTravel = '' THEN '9999999'
                    ELSE BusinessTravel
                END,
            DailyRate = CASE
                    WHEN DailyRate IS NULL OR DailyRate = '' THEN 9999999
                    ELSE DailyRate
                END,
            Department = CASE
                    WHEN Department IS NULL OR Department = '' THEN '9999999'
                    ELSE Department
                END,
            DistanceFromHome = CASE
                    WHEN DistanceFromHome IS NULL OR DistanceFromHome = '' THEN 9999999
                    ELSE DistanceFromHome
                END,
            Education = CASE
                    WHEN Education IS NULL OR Education = '' THEN 9999999
                    ELSE Education
                END,
            EducationField = CASE
                    WHEN EducationField IS NULL OR EducationField = '' THEN '9999999'
                    ELSE EducationField
                END,
            EmployeeCount = CASE
                    WHEN EmployeeCount IS NULL OR EmployeeCount = '' THEN 9999999
                    ELSE EmployeeCount
                END,
            EmployeeNumber = CASE
                    WHEN EmployeeNumber IS NULL OR EmployeeNumber = '' THEN 9999999
                    ELSE EmployeeNumber
                END
        """
    )


    conn.commit()  # Commit the update
    conn.close()
    return "Update Success"


def queryDelete():
    conn = sqlite3.connect("HR_1.db")
    cursor = conn.cursor()

    # Delete the record and commit the changes(Aga >= 18 
    # and Emp# 99999 which is test function of CREATE)
    cursor.execute("DELETE FROM HR_1 WHERE Age <= 18 OR EmployeeNumber = 99999")

    conn.commit()  # Commit the deletion
    conn.close()
    return "Delete Success"


if __name__ == "__main__":
    print(queryCreate())
    print(queryRead())
    print(queryUpdate())
    print(queryDelete())