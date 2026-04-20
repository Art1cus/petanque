import re
from datetime import datetime, timedelta

def replace_null_with_later_time(file_path):
    with open(file_path, 'r') as f:
        content = f.read()

    # This regex looks for the pattern of a single row in the VALUES clause.
    # It captures the start_datetime string in group 1.
    # It matches (..., 'start_datetime', [anything up to the closing parenthesis])
    pattern = r"(\(\d+,\s*\d+,\s*\d+,\s*\d+,\s*(?:TRUE|FALSE),\s*'[^']+')([^)]*)\)"

    def replacement_func(match):
        prefix = match.group(1)  # e.g., (1, 1, 43, 49, FALSE, '2025-04-25 13:30:00'
        middle = match.group(2)  # e.g., , NULL or , '2025-04-25 13:45:00'
        
        # Extract the datetime from the prefix (it's inside the last set of single quotes)
        dt_match = re.search(r"'([^']+)'", prefix)
        if not dt_match:
            return match.group(0)
        
        start_dt_str = dt_match.group(1).replace('CR', '')
        try:
            start_dt = datetime.strptime(start_dt_str, '%Y-%m-%d %H:%M:%S')
            end_dt = start_dt + timedelta(minutes=15)
            end_dt_str = end_dt.strftime('%Y-%m-%d %H:%M:%S')
            
            # Reconstruct the line with the new end_datetime
            # We use the prefix (which ends at the closing quote of start_datetime)
            # and append the comma, the new end_datetime in quotes, and the closing parenthesis.
            return f"{prefix}, '{end_dt_str}')"
        except Exception as e:
            print(f"Error processing datetime {start_dt_str}: {e}")
            return match.group(0)

    new_content = re.sub(pattern, replacement_func, content)

    with open(file_path, 'w') as f:
        f.write(new_content)
    print("Successfully updated init.sql")

if __name__ == "__main__":
    replace_null_with_later_time('/media/arthur/storage/Code/rust/petanque/backend/db/init.sql')
