import sys

def parse(srecords):
    binary = b""
    for srec in srecords:
        rectype = srec[:2]
        if rectype == "S0":
            addrlen = 2
        count = int(srec[2:4], base=16)

def translate(hexstr):
    binary = bytearray()
    for k in range(0, len(hexstr) // 2):
        binary.append(int(hexstr[2*k: 2*k+2], base=16))
    return binary


if __name__ == "__main__":
    try:
        progname = sys.argv[1]
    except IndexError:
        raise RuntimeError("Usage: ./srec2bin.py <hexfile>")
    with open(progname, "r") as f:
        srecords = [s.strip() for s in f.readlines()]
    binary = translate(srecords[0])
    # binary = parse(srecords)
    with open(f"{progname[:-4]}.bin", "wb") as f:
        f.write(binary)