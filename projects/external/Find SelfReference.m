(* ::Package:: *)

(* ::Section:: *)
(*Settings*)


SetDirectory@NotebookDirectory[];


(* ::Section:: *)
(*Functions*)


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


(* ::Section:: *)
(*Find Self Reference*)


char = Select[data, Equal @@ StringSplit[#Idiom, ""][[{1, -1}]]&];
pinyin = Select[data, Equal @@ StringSplit[#Pinyin, " "][[{1, -1}]]&];
letter = Select[data, Equal @@ StringSplit[#Letter, " "][[{1, -1}]]&];


$SelfCharacter := GeneralUtilities`Scope[
	$Removes = Sort@DeleteDuplicates@{
	
	};
	idioms = Query[DeleteCases[_?(MemberQ[$Removes , #Idiom]&)]]@char;
	Normal[Query[All, formatter]@idioms]
];
$SelfPinyin := GeneralUtilities`Scope[
	$Remove = Sort@DeleteDuplicates@{
	
	};
	idioms = Query[DeleteCases[_?(MemberQ[$Removes , #Idiom]&)]]@Complement[pinyin, char];
	Normal[Query[All, formatter]@idioms]
];
$SelfLetter := GeneralUtilities`Scope[
	$Remove = Sort@DeleteDuplicates@{
	
	};
	idioms = Query[DeleteCases[_?(MemberQ[$Removes , #Idiom]&)]]@Complement[letter, pinyin];
	Normal[Query[All, formatter]@idioms]
];


(* ::Section:: *)
(*Export*)


$ = GeneralUtilities`Scope[
	export = Import[Export["tmp.mx", #, "RawJSON"], "Text"]&;
	(*export=ExportString[#,"RawJSON"]&*)
	str = StringJoin[
		"export const ", "SelfReferenceCharacter", " = ",
		export@$SelfCharacter, "\n\n",
		"export const ", "SelfReferencePinyin", " = ",
		export@$SelfPinyin, "\n\n",
		"export const ", "SelfReferenceLetter", " = ",
		export@$SelfLetter
	];
	Export["SelfReference.ts", str, "Text"]
]

