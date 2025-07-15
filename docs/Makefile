.PHONY: rules.pdf

rules.pdf: rules.md
	pandoc rules.md -o rules.pdf \
	--pdf-engine=xelatex \
	-V 'mainfont:NotoSans-Regular.ttf' \
	-V 'mainfontoptions:BoldFont=NotoSans-Bold.ttf, ItalicFont=NotoSans-Italic.ttf, BoldItalicFont=NotoSans-BoldItalic.ttf' \
	-V 'geometry:margin=2cm' \
	-V 'header-includes:\pagenumbering{gobble}' \
	-V 'fontsize=12pt'
