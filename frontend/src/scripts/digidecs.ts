import {Result} from "@/scripts/core/result";
import {ApiError} from "@/scripts/core/error";
import {fetch1} from "@/scripts/core/fetch1";
import {server} from "@/main";

export enum DigidecsLocale {
  NL,
  EN,
}

export namespace DigidecsLocale {
  export function serverName(locale: DigidecsLocale): string {
    switch(locale) {
      case DigidecsLocale.EN: return "En"
      case DigidecsLocale.NL: return "Nl"
    }
  }
}

export class Digidecs {
  trackingId: string;
  attachments: string[]
  
  constructor(trackingId: string, attachments: string[]) {
    this.trackingId = trackingId;
    this.attachments = attachments;
  }
  
  static async start(
    name: string,
    iban: string,
    email: string,
    value: number,
    what: string,
    commission: string,
    notes: string | null,
    attachments: File[],
    locale: DigidecsLocale,
  ): Promise<Result<Digidecs, ApiError>> {
    const r = await fetch1(`${server}/api/digidecs/start`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
      },
      body: JSON.stringify({
        name: name,
        iban: iban.replaceAll(" ", ""),
        email: email,
        value: value,
        what: what,
        commission: commission,
        notes: notes,
        locale: DigidecsLocale.serverName(locale),
        attachments: attachments.map((att) => {
          return {
            name: att.name,
            mime: att.type,
          };
        }),
      })
    })

    if(r.isOk()) {
      interface StartedDigidecsResponse {
        tracking_id: string;
        attachments: {
          name: string,
          mime: string,
          tracking_id: string,
        }[]
      }
      
      
      return r.map1(async (response) => {
        const r = <StartedDigidecsResponse> await response.json();
        return new Digidecs(r.tracking_id, r.attachments.map((att) => att.tracking_id));
      });
    } else {
      return Result.err(r.unwrapErr());
    }
  }
  
  async upload_attachment(file: File, index: number): Promise<Result<[], ApiError>> {
    const r = await fetch1(`${server}/api/digidecs/attachment?tracking_id=${this.trackingId}&attachment_tracking_id=${this.attachments[index]}`, {
      method: 'POST',
      body: file,
    });
    
    if(r.isOk()) {
      return Result.ok([]);
    } else {
      return Result.err(r.unwrapErr());
    }
  }
  
  async complete(): Promise<Result<[], ApiError>> {
    const r = await fetch1(`${server}/api/digidecs/complete?tracking_id=${this.trackingId}`, {
      method: 'POST',
    });
    
    if(r.isOk()) {
      return Result.ok([]);
    } else {
      return Result.err(r.unwrapErr());
    }
  }
}
