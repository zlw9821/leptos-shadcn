import os

def combine_markdown_files(source_dir, output_file):
    with open(output_file, 'w', encoding='utf-8') as outfile:
        for root, dirs, files in os.walk(source_dir):
            for file in files:
                if file.endswith('.md'):
                    file_path = os.path.join(root, file)
                    # 写入文件路径开始标记和文件路径
                    outfile.write(f"-->{file_path}\n")
                    try:
                        with open(file_path, 'r', encoding='utf-8') as infile:
                            content = infile.read()
                            outfile.write(content)
                            outfile.write('\n')
                    except Exception as e:
                        print(f"Error reading {file_path}: {e}")
                    # 写入文件路径结束标记
                    outfile.write(f"-->{file_path} end\n")

if __name__ == "__main__":
    source_dir = "reference/book"
    output_file = "combined_markdown.md"
    combine_markdown_files(source_dir, output_file)
    print(f"All Markdown files combined into {output_file}")