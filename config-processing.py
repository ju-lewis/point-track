
CONFIG_FILE = "gnss-config.txt"

def main():

    with open(CONFIG_FILE, "r") as f:

        num_bytes = 0

        lines = f.readlines()
        print("{", end="")

        for line in lines:
            l = line.split(" - ")[1]

            for b in l.split(" "):
                num_bytes += 1
                print(f"0x{b}, ", end="")

        print("}")

        print("Bytes: " + str(num_bytes))



if __name__ == "__main__":
    main()
