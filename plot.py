import matplotlib.pyplot as plt
import json
import glob


def parse_log(filename):
    def parse_line(ln):
        ln = ln[1:]
        fields = {}
        for fs in ln.split("|"):
            k, v = fs.split("=")
            fields[k.strip()] = int(v)
        return fields

    bests = []
    with open(filename) as f:
        for ln in f.readlines():
            if ln.startswith("@"):
                fs = parse_line(ln)
                print(fs["best"])
                bests.append(fs['best'])
    return bests


for path in glob.glob("./short/*.txt"):
    filename = path.split('/')[-1]
    n_generation, n_population, n_elite, n_select, n_crossover, p_mutation = filename.split('.')[
        0].split('_')

    bests = parse_log(path)
    # print(bests)
    plt.semilogy(bests, label=filename)

plt.legend()
plt.ylim(top=3000)
plt.grid(True)
plt.savefig('plot.png')
