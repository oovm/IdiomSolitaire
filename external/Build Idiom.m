(* ::Package:: *)

(* ::Section:: *)
(*Setting*)


SetDirectory@NotebookDirectory[];


norm = RemoveDiacritics[#, Language -> "English"]&
StandardizeData[data_] := GeneralUtilities`Scope[
	char = First /@ data;
	pinyin = Map[StringRiffle@*StringSplit, data[[;;, 2]]];
	letter = norm /@ pinyin;
	Transpose@{char, pinyin, letter}
];


data = Import["database.csv", "CSV", "HeaderLines" -> 1];


data = Join[data[[All, ;; 2]], List /@ Map[norm, data[[All, 2]]], 2];
data = GroupBy[data, StringContainsQ[First@#, {"\:ff0c", ","}]&];


Export[
	"idiom.csv",
	First@data, "CSV",
	"TableHeadings" -> {"Character", "Pinyin", "Letter"},
	CharacterEncoding -> "UTF8"
]
