dot_file = "dot_bracket.txt"
unmod_file = "trna_unmodified.fasta"

string_arr = []


with open (unmod_file, "r") as f:
    lines = f.readlines()
    for l in lines:
        id = ""
        if l.startswith(">"):
            id = l[1:len(l)-1]
            string_arr.append(l[0:len(l)-1])
            with open(dot_file, "r") as f_dot:
                dot_lines = f_dot.readlines()
                for i in range(len(dot_lines)):
                    if dot_lines[i].startswith(">"):
                        if dot_lines[i][1:len(l)-1] == id:
                            string_arr.append(dot_lines[i+2][0:len(dot_lines[i+2])-1])
                f_dot.close()
        else:
            string_arr.append(l[0:len(l)-1])

        
if len(string_arr) > 0:
    with open("trna_unmodified_dot_bracket.txt", "w") as f:
        for string in string_arr:
            f.write(string)
            f.write("\n")
