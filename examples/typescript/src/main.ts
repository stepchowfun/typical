import { readFileSync, writeFileSync, unlinkSync } from 'fs';
import { Types, unreachable } from '../generated/types';

const requestFilePath = '/tmp/request';
const responseFilePath = '/tmp/response';
const { SendEmailRequest, SendEmailResponse } = Types;

function writeToFiles(): void {
  const requestMessage = {
    to: 'typical@example.com',
    subject: 'I love Typical!',
    body: 'It makes serialization easy and safe.',
  };

  const responseMessage = {
    error: 'Example error',
  };

  const requestArrayBuffer = SendEmailRequest.serialize(requestMessage);
  writeFileSync(requestFilePath, Buffer.from(requestArrayBuffer));

  const responseArrayBuffer = SendEmailResponse.serialize(responseMessage);
  writeFileSync(responseFilePath, Buffer.from(responseArrayBuffer));
}

function readFromFiles(): void {
  const requestFileContents = readFileSync(requestFilePath);
  const requestMessage = SendEmailRequest.deserialize(requestFileContents);
  if (requestMessage instanceof Error) {
    throw requestMessage;
  }

  const responseFileContents = readFileSync(responseFilePath);
  const responseMessage = SendEmailResponse.deserialize(responseFileContents);
  if (responseMessage instanceof Error) {
    throw responseMessage;
  }

  /* eslint-disable no-console -- Allow logging for this example. */

  console.log('to:', requestMessage.to);
  console.log('subject:', requestMessage.subject);
  console.log('body:', requestMessage.body);

  switch (responseMessage.$field) {
    case 'success':
      console.log('The email was sent!');
      break;
    case 'error':
      console.log('An error occurred:', responseMessage.error);
      break;
    default:
      return unreachable(responseMessage);
  }

  /* eslint-enable no-console -- Re-enable this rule. */

  return undefined; // To satisfy ESLint's `consistent-return` rule
}

writeToFiles();
readFromFiles();
unlinkSync(requestFilePath);
unlinkSync(responseFilePath);
