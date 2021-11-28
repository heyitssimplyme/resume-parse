# resume-parse

A small project to parse a resume using Lever's API and output useful information.

For each file there will be a JSON file saved to the current directory with the raw JSON data from the API.

## Usage
```
resume-parse -i
```
Launches interactive mode to select PDF files in the current folder.

```
resume-parse -f resume.pdf
```
Specifies the resume file to parse.
