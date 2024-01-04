// https://www.irs.gov/pub/irs-pdf/f8300.pdf

interface form8300senderComponents {
  revision: "December 2023";
  url: "https://www.irs.gov/pub/irs-pdf/f8300.pdf";
  identityOfIndividual: {
    lastName: string;
    middleIntitial: string;
    firstName: string;
    dateOfBirth: string;

    taxpayerIdentificationNumber: string;

    address: string;
    city: string;
    state: string;
    zip: string;
    country: string;

    occupationProfessionOrBusiness: string;
    identifyingDocument: {
      description: string;
      number: string;
      issuedBy: string;
    };
  };
}
