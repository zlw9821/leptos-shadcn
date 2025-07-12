import os


def combine_files(source_dir, output_file, file_extension):
    with open(output_file, 'w+', encoding='utf-8') as outfile:
        for root, _, files in os.walk(source_dir):
            for file in files:
                if file.endswith(file_extension):
                    file_path = os.path.abspath(os.path.join(root, file))
                    # 写入文件路径开始标记和文件路径
                    outfile.write(f"-->start {file_path}\n")
                    try:
                        with open(file_path, 'r', encoding='utf-8') as infile:
                            content = infile.read()
                            outfile.write(content)
                            outfile.write('\n')
                    except Exception as e:
                        print(f"Error reading {file_path}: {e}")
                    # 写入文件路径结束标记
                    outfile.write(f"-->end {file_path}\n")

if __name__ == "__main__":

    os.system("rm -f instructions.txt code.example.txt")

    source_dir = "../reference/"
    output_file = "instructions.txt"
    combine_files(source_dir, output_file, '.md')

    source_dir = "../src/"
    output_file = "examples.txt"
    combine_files(source_dir, output_file, '.rs')

    source_dir = "../reference/leptos/examples/"
    output_file = "examples.txt"
    combine_files(source_dir, output_file, '.rs')
