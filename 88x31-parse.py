file = "88x31.json"

import duckdb
import json
import os

db = duckdb.connect("88x31.ddb")

store = open(file)
j = json.load(store)

# Make sure that all tables are clean
#db.sql("DROP TABLE IF EXISTS buttons")
db.sql("DROP TABLE IF EXISTS pages")
#db.sql("DROP TABLE IF EXISTS backlinks")

"""
buttons = []
print("parsing links...")
for sha256, ext, names_idx, links_idx, backlinks_idx in list(
    zip(
        j["buttons"],
        j["button_file_exts"],
        j["button_names"],
        j["button_links"],
        j["button_backlinks"],
    )
):
    names = []
    for i in names_idx:
        names.append(j["texts"][i])
    links = []
    for l, b in zip(links_idx, backlinks_idx):
        links.append({"link": j["pages"][l], "backlink": j["pages"][b]})

    buttons.append({"sha256": sha256, "file_ext": ext, "names": names, "links": links})
print("writing to db...")
with open("TMPbuttons.json", "x") as f:
    f.write(json.dumps(buttons))

# Table containing all the 88x31s
# sha256: SHA256-hash of the button
# file_ext: the image filetype of the button
# names: a list of titles/alts that this button has
# (link VARCHAR, backlink VARCHAR): backlink is the page and link is the href on link
db.sql(
    "CREATE TABLE buttons AS SELECT * FROM read_json('TMPbuttons.json', columns={sha256: 'VARCHAR', file_ext: 'VARCHAR', names: 'VARCHAR[]', links: 'struct(link VARCHAR, backlink VARCHAR)[]'})"
)
os.remove("TMPbuttons.json")

print("parsing links done!")
"""
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
# page: the page which has this link
# link: where the page links to
# sha256: the SHA256-hash of the button
# alt: the alt-text of the button
# title: the title of the button
# filename: the original filename of the button
db.sql(
    "CREATE TABLE pages AS SELECT * FROM read_json('TMPpages.json', columns={page: 'VARCHAR', link: 'VARCHAR', sha256: 'VARCHAR', alt: 'VARCHAR', title: 'VARCHAR', filename: 'VARCHAR'})"
)
os.remove("TMPpages.json")

print("parsing pages done!")
"""
backlinks = []
print("parsing backlinks")
for page, backlinks_idx, backlink_buttons_idx in zip(
    j["pages"], j["backlinks"], j["backlink_buttons"]
):
    for p, b in zip(backlinks_idx, backlink_buttons_idx):
        link = j["pages"][p]
        sha256 = j["buttons"][b]
        backlinks.append({"page": page, "link": link, "sha256": sha256})
print("writing to db...")
with open("TMPbacklinks.json", "x") as f:
    f.write(json.dumps(backlinks))

# Table containing pages and which sites they link to with which button
# page: where the page links to
# link: which page links to the page
# sha2: the SHA2-hash of the button
db.sql(
    "CREATE TABLE backlinks AS SELECT * FROM read_json('TMPbacklinks.json', columns={page: 'VARCHAR', link: 'VARCHAR', sha256: 'VARCHAR'})"
)
os.remove("TMPbacklinks.json")
print("parsing backlinks done! all done!")
"""
print("all done")