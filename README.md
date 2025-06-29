# x227f-frontend
## usage
note: this requires a good amount of ram (2-3GB should work) as it loads all buttons into memory

0. install rust and python
1. run [x227f](https://github.com/mat-1/x227f) for some time, then rerun it with `FIX_IMAGES_MODE` set to `true`.
2. copy data/buttons and 88x31.json into the directory of x227f-frontend
3. install duckdb for python with `pip3 install duckdb`
4. build the database with `python3 88x31-parse.py`
5. update the contact info in static/index.html (or remove it if you don't care)
6. build and run the webserver using `cargo r -r`. this will take a while to build & load the buttons into memory. once it prints "listening [..]", it is running
7. access the frontend on :8831

if you need any help with this, feel free to contact me (see my github page)
