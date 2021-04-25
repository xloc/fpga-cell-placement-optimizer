import glob

for path in glob.glob("./output/*.txt"):
    filename = path.split('/')[-1]
    out_path = "./short/" + filename

    with open(path) as f:
        with open(out_path, 'w') as fw:
            for ln in f.readlines():
                if ln.startswith("@"):
                    fw.write(ln)
