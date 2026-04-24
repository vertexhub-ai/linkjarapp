import { useState } from 'react';
import { useMutation, useQueries, useQueryClient } from '@tanstack/react-query';
import { createLink, fetchStats, type CreateLinkResponse } from './api';

interface SavedLink {
  code: string;
  short_url: string;
}

function CopyButton({ text }: { text: string }) {
  const [copied, setCopied] = useState(false);

  function handleCopy() {
    navigator.clipboard.writeText(text).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    });
  }

  return (
    <button
      onClick={handleCopy}
      className="text-xs px-2 py-1 rounded bg-gray-100 hover:bg-gray-200 text-gray-600 transition-colors"
    >
      {copied ? 'Copied!' : 'Copy'}
    </button>
  );
}

export default function App() {
  const [url, setUrl] = useState('');
  const [links, setLinks] = useState<SavedLink[]>([]);
  const [lastResult, setLastResult] = useState<CreateLinkResponse | null>(null);
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: createLink,
    onSuccess(data) {
      setLastResult(data);
      setUrl('');
      setLinks(prev => {
        if (prev.some(l => l.code === data.code)) return prev;
        return [{ code: data.code, short_url: data.short_url }, ...prev];
      });
    },
  });

  const statsQueries = useQueries({
    queries: links.map(link => ({
      queryKey: ['stats', link.code],
      queryFn: () => fetchStats(link.code),
      staleTime: 30_000,
    })),
  });

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    const trimmed = url.trim();
    if (!trimmed) return;
    mutation.mutate(trimmed);
  }

  function handleRefresh(code: string) {
    queryClient.invalidateQueries({ queryKey: ['stats', code] });
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-2xl mx-auto px-4 py-12">
        <header className="mb-10">
          <h1 className="text-3xl font-bold text-gray-900">LinkJar</h1>
          <p className="text-gray-500 mt-1">Shorten URLs and track clicks</p>
        </header>

        {/* Shorten form */}
        <form onSubmit={handleSubmit} className="flex gap-2 mb-6">
          <input
            type="url"
            value={url}
            onChange={e => setUrl(e.target.value)}
            placeholder="https://example.com/some/long/url"
            required
            className="flex-1 rounded-lg border border-gray-300 px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
          <button
            type="submit"
            disabled={mutation.isPending}
            className="px-5 py-2.5 rounded-lg bg-blue-600 text-white text-sm font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {mutation.isPending ? 'Shortening…' : 'Shorten'}
          </button>
        </form>

        {/* Error */}
        {mutation.isError && (
          <div className="mb-4 rounded-lg bg-red-50 border border-red-200 px-4 py-3 text-sm text-red-700">
            {(mutation.error as Error).message}
          </div>
        )}

        {/* Success banner for most recent link */}
        {lastResult && (
          <div className="mb-8 rounded-lg bg-green-50 border border-green-200 px-4 py-4">
            <p className="text-xs text-green-600 font-medium mb-1">Link shortened!</p>
            <div className="flex items-center justify-between gap-2">
              <a
                href={lastResult.short_url}
                target="_blank"
                rel="noopener noreferrer"
                className="text-sm font-mono text-blue-700 hover:underline break-all"
              >
                {lastResult.short_url}
              </a>
              <CopyButton text={lastResult.short_url} />
            </div>
            <p className="text-xs text-green-700 mt-1">Code: <span className="font-mono">{lastResult.code}</span></p>
          </div>
        )}

        {/* Recent links list */}
        {links.length > 0 && (
          <section>
            <h2 className="text-sm font-semibold text-gray-500 uppercase tracking-wide mb-3">
              Recently shortened
            </h2>
            <ul className="space-y-3">
              {links.map((link, i) => {
                const query = statsQueries[i];
                const stats = query?.data;
                return (
                  <li
                    key={link.code}
                    className="rounded-lg border border-gray-200 bg-white px-4 py-4 shadow-sm"
                  >
                    <div className="flex items-start justify-between gap-3">
                      <div className="min-w-0">
                        <div className="flex items-center gap-2 mb-1">
                          <a
                            href={link.short_url}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-sm font-mono text-blue-700 hover:underline truncate"
                          >
                            {link.short_url}
                          </a>
                          <CopyButton text={link.short_url} />
                        </div>
                        <p className="text-xs text-gray-400 font-mono">
                          code: {link.code}
                        </p>
                        {stats?.url && (
                          <p className="text-xs text-gray-400 truncate mt-0.5">
                            → {stats.url}
                          </p>
                        )}
                      </div>
                      <div className="flex flex-col items-end gap-2 shrink-0">
                        <span className="inline-flex items-center gap-1 text-xs font-medium text-gray-700 bg-gray-100 rounded-full px-2.5 py-1">
                          {query?.isFetching ? (
                            <span className="text-gray-400">…</span>
                          ) : (
                            <>
                              <svg className="w-3 h-3 text-gray-500" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                                <path strokeLinecap="round" strokeLinejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                <path strokeLinecap="round" strokeLinejoin="round" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                              </svg>
                              {stats?.clicks ?? '–'}
                            </>
                          )}
                        </span>
                        <button
                          onClick={() => handleRefresh(link.code)}
                          className="text-xs text-gray-400 hover:text-blue-600 transition-colors"
                        >
                          Refresh
                        </button>
                      </div>
                    </div>
                  </li>
                );
              })}
            </ul>
          </section>
        )}
      </div>
    </div>
  );
}
