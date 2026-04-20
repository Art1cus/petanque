import csv
import html

def extract_teams(file_path):
    teams = set()
    try:
        with open(file_path, mode='r', encoding='utf-8') as csvfile:
            # The file uses ';' as a delimiter
            reader = csv.reader(csvfile, delimiter=';')
            # Skip header
            next(reader)
            for row in reader:
                if len(row) >= 5:
                    # Team names are in "Ploeg 1" (index 3) and "Ploeg 2" (index 4)
                    # We strip whitespace to ensure duplicates are correctly identified
                    team1 = html.escape(row[3].strip())
                    team2 = html.escape(row[4].strip())
                    if team1:
                        teams.add(team1)
                    if team2:
                        teams.add(team2)
        
        # Sort alphabetically
        sorted_teams = sorted(list(teams))
        return sorted_teams
    except Exception as e:
        print(f"Error: {e}")
        return []

if __name__ == "__main__":
    csv_file = 'Wedstrijdoverzicht.csv'
    result = extract_teams(csv_file)
    count = 1
    for team in result:
        print(f"{count} ('{team}', 1),")
        count += 1

