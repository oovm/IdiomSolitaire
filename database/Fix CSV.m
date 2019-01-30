(* ::Package:: *)

(* ::Section:: *)
(*Setting*)


SetDirectory@NotebookDirectory[];


(* ::Section:: *)
(*Import Fix*)


$replace = Sort@DeleteDuplicates@Import[
	"database-replace.csv",
	{"CSV", "Data"},
	"HeaderLines" -> 1,
	"IgnoreEmptyLines" -> True
];
Export[
	"database-replace.csv",
	$replace, "CSV",
	"TableHeadings" -> {"Idiom", "Pinyin", "Explanation", "Synonym"},
	"FillRows" -> False
];


$remove = Sort@DeleteDuplicates@Flatten@Import["database-remove.csv"];
Export[
	"database-remove.csv",
	Partition[DeleteDuplicates@$remove, UpTo[10]],
	"TextDelimiters" -> ""
];
