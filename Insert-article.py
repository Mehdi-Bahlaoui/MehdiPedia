from pathlib import Path
from datetime import datetime

file_path = "articles/js/articles-data.js"
after_string = "};"
current_date = datetime.now().strftime("%d/%m/%Y")

data = """
'ID': {
  title: 'TITLE',
  date: 'DATE',
  sections: [
    {
      id: 'introduction',
      title: '',
      content: 
      `
PLACEHOLDER-1

      `
    },
    {
      id: 'conclusion',
      title: '',
      content: 
      `
PLACEHOLDER-2

      `
    }
  ]
},
""".replace("DATE", current_date)


def insert_after_string(file_path: str, after: str, data: str) -> None:
    path = Path(file_path)
    text = path.read_text(encoding="utf-8")

    if after not in text:
        raise ValueError(f"'{after}' not found in file")

    # Find and remove the LAST occurrence of after_string
    last_index = text.rfind(after)
    if last_index == -1:
        raise ValueError(f"'{after}' not found in file")
    
    # Remove the last occurrence
    text_without_marker = text[:last_index] + text[last_index + len(after):]
    
    # Append data and put marker back
    new_text = text_without_marker + "\n" + data + "\n" + after

    path.write_text(new_text, encoding="utf-8")


if __name__ == "__main__":
    insert_after_string(file_path, after_string, "\n" + data)
    print("Inserted data successfully.")
