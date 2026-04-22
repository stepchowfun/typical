import runCircularDependency from './circular-dependency';
import runComprehensive from './comprehensive';
import runDegenerate from './degenerate';
import runSchemaEvolution from './schema-evolution';
import { verifyOmnifile } from './assertions';

document.querySelector<HTMLDivElement>('#app')!.innerHTML =
  'Running integration tests\u2026';

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

  document.querySelector<HTMLDivElement>('#app')!.innerHTML =
    'Integration tests passed.';
});
