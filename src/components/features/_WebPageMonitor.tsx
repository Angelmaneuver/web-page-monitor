import { CSSProperties, Suspense, use, useState } from 'react';
import type { FallbackProps } from 'react-error-boundary';
import { ErrorBoundary } from 'react-error-boundary';

import { Spinner } from '@/components/ui/spinner';
import { getAppLabel, reload } from '@/lib/tauri/window';

import Window from './_Window';

function WebPageMonitor() {
  const [initial, setInitial] = useState(getAppLabel());

  return (
    <ErrorBoundary FallbackComponent={ErrorFallback} onReset={() => setInitial(getAppLabel())}>
      <Suspense
        fallback={
          <section className="loading">
            <Spinner />
          </section>
        }
      >
        <Wrapper initial={initial} />
      </Suspense>
    </ErrorBoundary>
  );
}

function Wrapper({ initial }: { initial: Promise<string> }) {
  const data = use(initial);

  return <Window label={data} reload={reload} />;
}

function ErrorFallback({ error, resetErrorBoundary }: FallbackProps) {
  return (
    <Window label="" reload={async () => resetErrorBoundary()}>
      <div className="main">
        <LED style={{ width: '28.9em' }} text={`エラーが発生しました:${toString(error)}`} />
      </div>
    </Window>
  );
}

function toString(error: unknown): string {
  if (typeof error === 'string') {
    return (error as string).trim();
  } else if (error instanceof Error) {
    return error.message;
  } else {
    return '不明なエラー';
  }
}

function LED({
  className = '',
  style = undefined,
  marquee = true,
  text = '',
}: {
  className?: string;
  style?: CSSProperties;
  marquee?: boolean;
  text?: string;
}) {
  return (
    <div className={`led ${marquee ? 'marquee' : ''} ${className || ''}`.trim()} style={style}>
      {text.length > 0 ? (
        <span style={marquee ? { animationDuration: `${text.length}s` } : undefined}>{text}</span>
      ) : (
        ''
      )}
    </div>
  );
}

LED.defaultProps = {
  className: '',
  style: undefined,
  marquee: true,
  text: '',
};

export default WebPageMonitor;
