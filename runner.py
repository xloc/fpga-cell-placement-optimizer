import os


# def parse_line(ln):
#     ln = ln[1:]
#     fields = {}
#     for fs in ln.split("|"):
#         k, v = fs.split("=")
#         fields[k.strip()] = int(v)
#     return fields


# def append_record(filename, key, new_record):
#     try:
#         open(filename)
#     except FileNotFoundError:
#         with open(filename, "w") as f:
#             f.write("{}")

#     import json
#     with open(filename) as f:
#         rec = json.load(f)

#     rec[key] = new_record

#     with open(filename, "w") as f:
#         json.dump(rec, f)


record_filename = "records.json"

n_generation = 100_000
n_population = 100
n_elite = 10
n_select = 20
n_crossover = (n_population-n_select) // 2
p_mutation = 0.5

for n_elite in [19]:
    args_str = "{} {} {} {} {} {}".format(
        n_generation, n_population, n_elite, n_select, n_crossover, p_mutation)

    filename = "{}.txt".format(args_str.replace(" ", "_"))

    os.system(f"cargo r --release -- {args_str} >> {filename}")


# append_record(record_filename, args_str, bests)
