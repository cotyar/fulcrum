using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Fulcrum;
using LanguageExt;

namespace Falcrum
{
    public enum AddResult
    {
        Success,
        Exists
    }
    
    public enum DeleteResult
    {
        Success,
        NotFound
    }
    
    public class SearchResult<TKey, TValue>: Record<SearchResult<TKey, TValue>>
    {
        public readonly TKey Key;
        public readonly TValue Value;
        public readonly Option<KvMetadata> KvMetadata;

        public SearchResult(TKey key, TValue val, Option<KvMetadata> kvMetadata)
        {
            Key = key;
            Value = val;
            KvMetadata = kvMetadata;
        }
    }
    
    public interface IClient<TKey, TValue>
    {
        ValueTask<Either<AddResult, InternalError>> Add(TKey key, TValue val, bool overrideIfExists);
        ValueTask<Either<DeleteResult, InternalError>> Delete(TKey key);
        
        ValueTask<Either<bool, InternalError>> Contains(TKey key);
        ValueTask<Either<Option<TValue>, InternalError>> Get(TKey key);

        IAsyncEnumerable<Either<SearchResult<TKey, TValue>, (Option<InternalError>, Option<InternalError>)>> Search(TKey from, bool includeMetadata = false); // TODO: Add reverse search
    }
}