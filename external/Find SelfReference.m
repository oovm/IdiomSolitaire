(* ::Package:: *)

SetDirectory@NotebookDirectory[];


data = Import["database.csv", {"CSV", "Dataset"}, "HeaderLines" -> { 1, 1}];
idiom = Import["idiom.csv", {"CSV", "Dataset"}, "HeaderLines" -> 1];


char = Select[idiom, Equal @@ StringSplit[#[[1]], ""][[{1, -1}]]&];
pinyin = Select[idiom, Equal @@ StringSplit[#[[2]], " "][[{1, -1}]]&];
letter = Select[idiom, Equal @@ StringSplit[#[[3]], " "][[{1, -1}]]&];


$SelfCharacter = GeneralUtilities`Scope[
	$Remove = Sort@DeleteDuplicates@{
		"\:5e8a\:4e0a\:53e0\:5e8a", "\:5e8a\:4e0a\:8fed\:5e8a", "\:5e8a\:4e0a\:65bd\:5e8a", "\:5e8a\:4e0b\:5b89\:5e8a", "\:8bb9\:4ee5\:6ecb\:8bb9", "\:9632\:4e0d\:53ca\:9632", "\:89c1\:6240\:4e0d\:89c1",
		"\:95fb\:6240\:4e0d\:95fb", "\:4eb2\:4e0a\:6210\:4eb2", "\:65e5\:751a\:4e00\:65e5", "\:65e5\:614e\:4e00\:65e5", "\:5c4b\:4e0b\:4f5c\:5c4b", "\:65b0\:76ca\:6c42\:65b0", "\:679d\:5916\:751f\:679d",
		"\:60fa\:60fa\:60dc\:60fa\:60fa"
	};
	idioms = DeleteCases[Normal[First /@ char], Alternatives @@ $Remove];
	Map[Prepend[Values@#, "Idiom" -> Keys@#]&, Normal@Normal@Query[idioms, All]@data]
];
$SelfPinyin = GeneralUtilities`Scope[
	$Remove = Sort@DeleteDuplicates@{
		"\:552f\:6240\:6b32\:4e3a", "\:60df\:6240\:6b32\:4e3a",
		"\:5343\:91cc\:59fb\:7f18\:4f7f\:7ebf\:7275"
	};
	idioms = DeleteCases[Normal[First /@ Complement[pinyin, char]], Alternatives @@ $Remove];
	Map[Prepend[Values@#, "Idiom" -> Keys@#]&, Normal@Normal@Query[idioms, All]@data]
];
$SelfLetter = GeneralUtilities`Scope[
	$Remove = Sort@DeleteDuplicates@{
		"\:4e00\:52b3\:4e45\:9038", "\:60df\:6240\:6b32\:4e3a", "\:9732\:7ea2\:70df\:7eff", "\:9e6d\:670b\:9e25\:4fa3",
		"\:5343\:91cc\:59fb\:7f18\:4f7f\:7ebf\:7275"
	};
	idioms = DeleteCases[Normal[First /@ Complement[letter, pinyin]], Alternatives @@ $Remove];
	Map[Prepend[Values@#, "Idiom" -> Keys@#]&, Normal@Normal@Query[idioms, All]@data]
];


$ = GeneralUtilities`Scope[
	export = Import[Export["tmp.mx", #, "RawJSON"], "Text"]&;
	(*export=ExportString[#,"RawJSON"]&*)
	str = StringJoin[
		"export const ", "Character", " = ",
		export@$SelfCharacter, "\n\n",
		"export const ", "Pinyin", " = ",
		export@$SelfPinyin, "\n\n",
		"export const ", "Letter", " = ",
		export@$SelfLetter
	];
	Export["SelfReference.ts", str, "Text"]
]
