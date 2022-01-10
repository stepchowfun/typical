import runCircularDependency from './circular-dependency';
import runComprehensive from './comprehensive';
import runDegenerate from './degenerate';
import runSchemaEvolution from './schema-evolution';
import { verifyOmnifile } from './assertions';

const startingParagraph = document.createElement('p');
startingParagraph.innerHTML = 'Running integration tests\u2026';
document.body.appendChild(startingParagraph);

window.requestAnimationFrame(() => {
  try {
    console.log('Running circular dependency integration test\u2026\n');
    runCircularDependency();

    console.log('\nRunning comprehensive integration test\u2026\n');
    runComprehensive();

    console.log('\nRunning degenerate integration test\u2026\n');
    runDegenerate();

    console.log('\nRunning schema evolution integration test\u2026\n');
    runSchemaEvolution();

    console.log('\nVerifying omnifile\u2026\n');
    verifyOmnifile();
  } catch (e) {
    const failureParagraph = document.createElement('p');
    failureParagraph.innerHTML =
      'Integration tests failed. See the console for details.';
    document.body.appendChild(failureParagraph);

    throw e;
  }

  const successParagraph = document.createElement('p');
  successParagraph.innerHTML = 'Integration tests passed.';
  document.body.appendChild(successParagraph);
});
