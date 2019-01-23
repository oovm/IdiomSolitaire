(* ::Package:: *)

(* ::Section:: *)
(*Setting*)


SetDirectory@NotebookDirectory[];


(* ::Section:: *)
(*Data*)


data = GeneralUtilities`Scope[
	r := URLDownload[
		"https://github.com/by-syk/chinese-idiom-db/raw/master/chinese-idioms-12976.txt",
		"Source_1.mx"
	];
	If[!FileExistsQ@"Source_1.mx", r];
	tmp = Import["Source_1.mx", "CSV"];
	Echo[Length@tmp, "Records:"];
	tmp[[All, {2, 3, 4}]]
];


data = MapAt[StringRiffle@*StringSplit, data, {All, 2}]


(* ::Section:: *)
(*Export*)


(* ::Subsection:: *)
(*Export*)


Export[
	"database.csv",
	SortBy[data, Rest], "CSV",
	"TableHeadings" -> {"Idiom", "Pinyin", "Explanation"},
	CharacterEncoding -> "UTF8"
]


(* ::Subsection:: *)
(*Export Accelerate*)


(* ::Input:: *)
(*string = ExportString[*)
(*	SortBy[data, Rest], "CSV",*)
(*	"TableHeadings" -> {"Idiom", "Pinyin", "Explanation"},*)
(*	CharacterEncoding -> "UTF8"*)
(*];*)
(*Export["database.csv", string, "Table", CharacterEncoding -> "UTF8"]*)
