import { _SERVICE as Main, ThirdPartyRequest, ThirdPartyResponse } from "../../../declarations/main/main.did";

export const kinds = [
    { value: 'Hospital', label: 'Hospital' },
    { value: 'DrugStore', label: 'Drug store' },
    { value: 'Other', label: 'Other' },
  ];

  export const thirdPartyCreate = async (
    main: Main,
    req: ThirdPartyRequest
): Promise<ThirdPartyResponse> => {
    const res = await main.thirdparty_create(req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
}