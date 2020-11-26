import * as S from "./SelfReference.js";
import * as K from "./Killer.js"

export const SelfReference = {
    Character: S.SelfReferenceCharacter.map(x => x.Idiom),
    Pinyin: S.SelfReferencePinyin.map(x => x.Idiom),
    Letter: S.SelfReferenceLetter.map(x => x.Idiom),
}

export const Killer = {
    Character: K.KillCharacter,
    Pinyin: K.KillPinyin.map(x => x.Idiom),
    Letter: K.KillLetter.map(x => x.Idiom),
}
