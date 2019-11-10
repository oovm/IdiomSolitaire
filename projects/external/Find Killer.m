(* ::Package:: *)

(* ::Section:: *)
(*Settings*)


SetDirectory@NotebookDirectory[];


(* ::Section:: *)
(*Functions*)


orderedComplement[all_List, i__List] := DeleteDuplicates[Join @ ##] ~ Drop ~ Length[#] &[Union @ i, DeleteDuplicates @ all]
getKiller = orderedComplement[Keys@GroupBy[#, Last], Keys@GroupBy[#, First]]&;
formatter = <|
	"Idiom" -> #Idiom,
	"Pinyin" -> #Pinyin,
	"Explain" -> #Explanation,
	If[#Synonym == "", Nothing, "Synonym" -> #Synonym]
|>&;


(* ::Section:: *)
(*Loading*)


data = Import["database.csv", {"CSV", "Dataset"}, "HeaderLines" -> 1];
data = Select[data, !StringContainsQ[#Idiom, {"\:ff0c", ","}]&];
data = SortBy[data, Last@StringSplit[#Pinyin, " "]&];


(* ::Section:: *)
(*Find Killer*)


char = Normal[Query[All, StringSplit[#Idiom, ""]&]@data];
pinyin = Normal[Query[All, StringSplit[#Pinyin, " "]&]@data];
letter = Normal[Query[All, StringSplit[#Letter, " "]&]@data];


Clear[$KillCharacter, $KillPinyin, $KillLetter]
$KillLetter := $KillLetter = GeneralUtilities`Scope[
	$Removes = {"\:4e1c\:8865\:897f\:51d1", "\:4e1c\:632a\:897f\:51d1", "\:4e0d\:54fc\:4e0d\:54c8", "\:91cd\:751f\:7237\:5a18"};
	killer = Function[c, Normal@Select[data  , Last@StringSplit[#Letter, " "] == c&]];
	words = DeleteCases[#Idiom& /@ Flatten[killer /@ getKiller@letter ], Alternatives @@ $Removes];
	export = Query[DeleteCases[_?(!MemberQ[words, #Idiom]&)]]@data;
	Normal[Query[All, formatter]@ export]
];
$KillPinyin := $KillPinyin = GeneralUtilities`Scope[
	$Removes = {"\:4e1c\:8865\:897f\:51d1", "\:4e1c\:632a\:897f\:51d1", "\:4e0d\:54fc\:4e0d\:54c8", "\:91cd\:751f\:7237\:5a18"};
	killer = Function[c, Normal@Select[data  , Last@StringSplit[#Pinyin, " "] == c&]];
	words = DeleteCases[#Idiom& /@ Flatten[killer /@ getKiller@pinyin ], Alternatives @@ $Removes];
	export = Query[DeleteCases[_?(!MemberQ[words, #Idiom]&)]]@data;
	Complement[Normal[Query[All, formatter]@ export], $KillLetter]
];
$KillCharacter := $KillCharacter = GeneralUtilities`Scope[
	$Removes = {"\:4e1c\:8865\:897f\:51d1", "\:4e1c\:632a\:897f\:51d1", "\:4e0d\:54fc\:4e0d\:54c8", "\:91cd\:751f\:7237\:5a18"};
	killer = Function[c, Normal@Select[data  , Last@StringSplit[#Idiom, ""] == c&]];
	words = DeleteCases[#Idiom& /@ Flatten[killer /@ getKiller@char ], Alternatives @@ $Removes];
	export = Query[DeleteCases[_?(!MemberQ[words, #Idiom]&)]]@data;
	Complement[Normal[Query[All, formatter]@ export], $KillPinyin]
];


Clear[$KillCharacter, $KillPinyin, $KillLetter]
$KillLetter := $KillLetter = GeneralUtilities`Scope[
	formatter = <|
		"Idiom" -> #Idiom,
		"Pinyin" -> #Pinyin,
		"Explain" -> #Explanation,
		If[#Synonym == "", Nothing, "Synonym" -> #Synonym]
	|>&;
	$Removes = {"\:4e1c\:8865\:897f\:51d1", "\:4e1c\:632a\:897f\:51d1", "\:4e0d\:54fc\:4e0d\:54c8", "\:91cd\:751f\:7237\:5a18"};
	killer = Function[c, Normal@Select[data  , Last@StringSplit[#Letter, " "] == c &]];
	words = DeleteCases[#Idiom & /@ Flatten[killer /@ getKiller@letter ], Alternatives @@ $Removes];
	export = Query[DeleteCases[_?(! MemberQ[words, #Idiom] &)]]@data;
	Normal[Query[All, formatter]@ export]
];
$KillPinyin := $KillPinyin = With[
	{k = getKiller@pinyin},
	formatter = <|
		"Idiom" -> #Idiom,
		"Pinyin" -> #Pinyin,
		If[#Synonym == "", Nothing, "Synonym" -> #Synonym]
	|>&;
	
	index = GeneralUtilities`SelectIndices[pinyin, MemberQ[k, Last@#]&];
	export = orderedComplement[Normal[Query[All, formatter][data[[index]]]], $KillLetter]
];
$KillCharacter := $KillCharacter = With[
	{k = getKiller@char },
	formatter = #Idiom&;
	index = GeneralUtilities`SelectIndices[char, MemberQ[k, Last@#]&];
	export = orderedComplement[Normal[Query[All, formatter][data[[index]]]], $KillPinyin]
];


Clear[$KillCharacter, $KillPinyin, $KillLetter]
$KillLetter := $KillLetter = GeneralUtilities`Scope[
	formatter = <|
		"Idiom" -> #Idiom,
		"Pinyin" -> #Pinyin,
		"Explain" -> #Explanation,
		If[#Synonym == "", Nothing, "Synonym" -> #Synonym]
	|>&;
	$Removes = {};
	killer = Function[c, Normal@Select[data  , Last@StringSplit[#Letter, " "] == c &]];
	words = DeleteCases[#Idiom & /@ Flatten[killer /@ getKiller@letter ], Alternatives @@ $Removes];
	export = Query[DeleteCases[_?(! MemberQ[words, #Idiom] &)]]@data;
	Normal[Query[All, formatter]@ export]
];
$KillPinyin := $KillPinyin = With[
	{k = getKiller@pinyin},
	formatter = <|
		"Idiom" -> #Idiom,
		"Pinyin" -> #Pinyin,
		If[#Synonym == "", Nothing, "Synonym" -> #Synonym]
	|>&;
	
	index = GeneralUtilities`SelectIndices[pinyin, MemberQ[k, Last@#]&];
	export =Query[All, formatter][data[[index]]];
	Normal[Query[DeleteCases[_?( MemberQ[First/@$KillLetter, #Idiom] &)]]@export ]
];
$KillCharacter := $KillCharacter = Block[
	{
	k = getKiller@char,
	formatter = #Idiom& ,
	$Removes=Join[First/@$KillPinyin,First/@$KillLetter],
	index ,export
	},
	index = GeneralUtilities`SelectIndices[char, MemberQ[k, Last@#]&];
	export = Normal[Query[All, formatter][data[[index]]]];
DeleteCases[export ,Alternatives@@$Removes]
];


(* ::Section:: *)
(*Export*)


$ = GeneralUtilities`Scope[
	exporter = Import[Export["tmp.mx", #, "RawJSON"], "Text"]&;
	(*export=ExportString[#,"RawJSON"]&*)
	chars = Partition[Flatten@Values@GroupBy[$KillCharacter, StringLength], UpTo[10]];
	str = StringJoin[
		"export const ", "KillCharacter", " = ", "[\n\"",
		StringRiffle[StringRiffle[#, "\",\""]& /@ chars, "\",\n\""],
		"\"\n]", "\n\n",
		"export const ", "KillPinyin", " = ",
		exporter@$KillPinyin, "\n\n",
		"export const ", "KillLetter", " = ",
		exporter@$KillLetter
	];
	Export["Killer.ts", str, "Text"]
]
