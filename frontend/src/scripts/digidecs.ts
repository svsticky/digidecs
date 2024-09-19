import {Result} from "@/scripts/core/result";
import {ApiError} from "@/scripts/core/error";
import {fetch1} from "@/scripts/core/fetch1";
import {server} from "@/main";
import {encode} from "base64-arraybuffer";

export class Digidecs {
  static async digidecs(
    name: string,
    iban: string,
    email: string,
    value: number,
    what: string,
    commission: string,
    notes: string,
    attachments: File[],
  ): Promise<Result<[], ApiError>> {
    const attachmentsBase64 = await Promise.all(attachments.map(async attachment => {
      const buffer = await attachment.arrayBuffer();
      // const content = btoa(String.fromCharCode(...new Uint8Array(buffer)));
      const content = encode(buffer);
      return {
        content: content,
        name: attachment.name,
        mime: attachment.type,
      }
    }));

    const r = await fetch1(`${server}/api/digidecs`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
      },
      body: JSON.stringify({
        name: name,
        iban: iban,
        email: email,
        value: value,
        what: what,
        commission: commission,
        notes: notes,
        attachments: attachmentsBase64,
      })
    });

    if(r.isOk()) {
      return Result.ok([]);
    } else {
      return Result.err(r.unwrapErr());
    }
  }
}