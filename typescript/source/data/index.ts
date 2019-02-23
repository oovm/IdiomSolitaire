import * as S from "./SelfReference";
import * as K from "./Killer"

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

console.log(SelfReference)

console.log(Killer)