file = "88x31.json"

import duckdb
import json
import os

db = duckdb.connect("88x31.ddb")

store = open(file)
j = json.load(store)

# Make sure that all tables are clean
db.sql("DROP TABLE IF EXISTS pages")

pages = []
print("parsing pages")
for (
    page,
    links_idx,
    buttons_idx,
    button_alts_idx,
    button_titles_idx,
    button_filenames_idx,
) in zip(
    j["pages"],
    j["links"],
    j["link_buttons"],
    j["link_button_alts"],
    j["link_button_titles"],
    j["link_button_filenames"],
):
    for l, b, ba, bt, bf in zip(
        links_idx, buttons_idx, button_alts_idx, button_titles_idx, button_filenames_idx
    ):
        link = j["pages"][l] if l is not None else None
        sha256 = j["buttons"][b]
        alt = j["texts"][ba] if ba is not None else None
        title = j["texts"][bt] if bt is not None else None
        filename = j["texts"][bf] if bf is not None else None
        pages.append(
            {
                "page": page,
                "link": link,
                "sha256": sha256,
                "alt": alt,
                "title": title,
                "filename": filename,
            }
        )
print("writing to db...")
with open("TMPpages.json", "x") as f:
    f.write(json.dumps(pages))

# Table containing records of pages and what which buttons they contain (one page->button per record)
db.sql(
    "CREATE TABLE pages AS SELECT * FROM read_json('TMPpages.json', columns={page: 'VARCHAR', link: 'VARCHAR', sha256: 'VARCHAR', alt: 'VARCHAR', title: 'VARCHAR', filename: 'VARCHAR'})"
)
os.remove("TMPpages.json")

print("parsing pages done!")
