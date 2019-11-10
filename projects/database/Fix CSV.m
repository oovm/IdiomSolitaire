(* ::Package:: *)

(* ::Section:: *)
(*Setting*)


SetDirectory@NotebookDirectory[];


(* ::Section:: *)
(*Functions*)


norm = RemoveDiacritics[#, Language -> "English"]&;
addLetter = <|
	"Idiom" -> #Idiom,
	"Pinyin" -> #Pinyin,
	"Letter" -> norm[#Pinyin],
	"Explanation" -> #Explanation,
	"Synonym" -> #Synonym
|>&;


(* ::Section:: *)
(*Import Fix*)


$replace = GeneralUtilities`Scope[
	import = Import[
		"database-replace.csv",
		{"CSV", "Data"},
		"HeaderLines" -> 1,
		"IgnoreEmptyLines" -> True
	];
	export = Sort@DeleteDuplicates@import;
	Export[
		"database-replace.csv",
		export, "CSV",
		"TableHeadings" -> {"Idiom", "Pinyin", "Explanation", "Synonym"},
		"FillRows" -> False
	];
	Return[export]
];


$remove = GeneralUtilities`Scope[
	import = Import["database-remove.csv"];
	add = StringSplit[Last@#, "|"]& /@ $replace;
	export = Sort@DeleteDuplicates@Flatten@Join[First /@ $replace, import, add];
	Export[
		"database-remove.csv",
		Partition[DeleteCases[DeleteDuplicates@export, ""], UpTo[10]],
		"TextDelimiters" -> ""
	];
	Return[export]
];


(* ::Section:: *)
(*Apply Fix*)


$base = Import["database-base.csv", {"CSV", "Dataset"}, "HeaderLines" -> 1];
data = Query[DeleteCases[_?(MemberQ[$remove, #Idiom]&)]]@$base;
import = Import["database-replace.csv", {"CSV", "Dataset"}, "HeaderLines" -> 1];
export = Dataset@SortBy[Join[data, Normal@import], #Pinyin&];
Export[
	FileNameJoin[{ParentDirectory[NotebookDirectory[]], "external", "database.csv"}],
	Query[All, addLetter]@export,
	CharacterEncoding -> "UTF8"
]
