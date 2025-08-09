def convert_font_file(input_file="font.txt", output_file="font_output.txt"):
    try:
        with open(input_file, 'r') as f:
            content = f.read()

        # Split the content into character blocks
        # Assuming each character block is separated by blank lines
        blocks = [block.strip() for block in content.split('\n\n') if block.strip()]

        output = []

        for block in blocks:
            lines = block.strip().split('\n')

            # Assuming the first line contains the character
            # For example: "A:" or just "A"
            char_line = lines[0].strip()

            # Try to get the character, assuming it's the first non-whitespace character
            char = char_line[0]
            if char in ("'", "\\"):
                char = f"\\{char}"

            # Get the data lines (skip the first line which contains the character)
            data_lines = [line.strip() for line in lines[1:] if line.strip()]

            # Format the output in Rust format
            rust_block = f"b'{char}' => Char {{\n    data: [\n"
            for data_line in data_lines:
                rust_block += f'        "{data_line}",\n'
            rust_block += "    ],\n},"

            output.append(rust_block)

        # Write the output to a file
        with open(output_file, 'w') as f:
            f.write('\n\n'.join(output))

        print(f"Conversion completed. Output written to {output_file}")

    except FileNotFoundError:
        print(f"Error: File '{input_file}' not found.")

if __name__ == "__main__":
    convert_font_file()